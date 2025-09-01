use glam::Affine3A;
use nif::*;
use str_utils::*;

use easy_ext::ext;

pub type Geometry<'a> = (&'a NiTriShape, &'a NiTriShapeData, Affine3A);

#[ext]
impl NiStream {
    pub fn collision_geometries(&self) -> impl Iterator<Item = Geometry<'_>> {
        let mut root = self.roots.first().copied().unwrap_or_default();
        let mut root_transform = Affine3A::IDENTITY;

        let mut skip_culled = true;
        let mut has_markers = false;
        let mut has_nested_collsion_node = false;

        if self.root_has_string_data_starting_with("nc") {
            root = NiLink::null();
        } else {
            if self.root_has_string_data_starting_with("mrk") {
                has_markers = true;
            }
            if self.root_has_string_data_starting_with("rcn") {
                has_nested_collsion_node = true;
            }
        }

        // If a collision node exists, prefer that as the root.
        // Note: You must include appCulled nodes in this case.
        if let Some(node) = self.get_as::<_, NiNode>(root) {
            let option: Option<&RootCollisionNode> = if has_nested_collsion_node {
                node.children_of_type_recursive(self).next()
            } else {
                node.children_of_type(self).next()
            };
            if let Some(collision) = option {
                root = self.get_link(collision).cast();
                root_transform = collision.transform(); // If nested should this be world transform?
                skip_culled = false;
            }
        }

        let mut stack = vec![(root.key, root_transform)];

        std::iter::from_fn(move || {
            while let Some((key, transform)) = stack.pop() {
                let Some(object) = self.objects.get(key) else {
                    continue;
                };

                if object.is_instance_of::<NiDynamicEffect>() || object.is_instance_of::<NiBillboardNode>() {
                    continue;
                }

                if let Ok(node) = <&NiBSParticleNode>::try_from(object)
                    && node.follow()
                {
                    continue;
                }

                if let Ok(node) = <&NiCollisionSwitch>::try_from(object)
                    && !node.propagate()
                {
                    continue;
                }

                if let Ok(object) = <&NiAVObject>::try_from(object) {
                    if skip_culled && object.app_culled() {
                        // TODO: Do culled objects influence collision when not using RootCollisionNode?
                        continue;
                    }
                    if has_markers && is_editor_marker(object) {
                        // Note: we use intent over correctness here by discarding nested editor markers.
                        continue;
                    }
                }

                if let Ok(node) = <&NiSwitchNode>::try_from(object) {
                    let transform = transform * node.transform();
                    if let Some(child) = node.children.get(node.active_index) {
                        stack.push((child.key, transform));
                    }
                    continue;
                }

                if let Ok(node) = <&NiNode>::try_from(object) {
                    let transform = transform * node.transform();
                    for child in node.children.iter().rev() {
                        stack.push((child.key, transform));
                    }
                    continue;
                }

                if let Ok(shape) = <&NiTriShape>::try_from(object)
                    && let Some(data) = self.get_as(shape.geometry_data)
                {
                    let transform = transform * shape.transform();
                    return Some((shape, data, transform));
                }
            }
            None
        })
    }

    pub fn visible_geometries(&self) -> impl Iterator<Item = Geometry<'_>> {
        let root = self.roots.first().copied().unwrap_or_default();
        let has_markers = self.root_has_string_data_starting_with("mrk");

        let mut stack = vec![(root.key, Affine3A::IDENTITY)];

        std::iter::from_fn(move || {
            while let Some((key, transform)) = stack.pop() {
                let Some(this) = self.objects.get(key) else {
                    continue;
                };

                if this.is_instance_of::<NiDynamicEffect>()
                    || this.is_instance_of::<NiBillboardNode>()
                    || this.is_instance_of::<NiBSParticleNode>()
                    || this.is_instance_of::<RootCollisionNode>()
                {
                    continue;
                }

                if let Ok(object) = <&NiAVObject>::try_from(this)
                    && (object.app_culled() || (has_markers && is_editor_marker(object)))
                {
                    continue;
                }

                if let Ok(node) = <&NiSwitchNode>::try_from(this) {
                    let transform = transform * node.transform();
                    if let Some(child) = node.children.get(node.active_index) {
                        stack.push((child.key, transform));
                    }
                    continue;
                }

                if let Ok(node) = <&NiNode>::try_from(this) {
                    let transform = transform * node.transform();
                    for child in node.children.iter().rev() {
                        stack.push((child.key, transform));
                    }
                    continue;
                }

                if let Ok(shape) = <&NiTriShape>::try_from(this)
                    && shape.skin_instance.is_null()
                    && let Some(data) = self.get(shape.geometry_data.cast::<NiTriShapeData>())
                    && data.bound.radius > 1e-6
                {
                    let transform = transform * shape.transform();
                    return Some((shape, data, transform));
                }
            }
            None
        })
    }

    pub fn clear_root_node_transforms(&mut self) {
        for root in &self.roots {
            if let Some(object) = self.objects.get_mut(root.key)
                && let Ok(node) = <&mut NiNode>::try_from(object)
            {
                node.clear_transform();
            }
        }
    }

    pub fn discard_editor_markers(&mut self) {
        if self.root_has_string_data_starting_with("mrk") {
            self.objects.retain(|_, object| {
                let Ok(object) = <&NiAVObject>::try_from(&*object) else {
                    return true;
                };
                object
                    .name
                    .starts_with_ignore_ascii_case_with_lowercase_multiple(&["editormarker", "tri editormarker"])
                    .is_none()
            })
        }
    }

    pub fn flatten_properties(&mut self) {
        use enum_map::{Enum, EnumMap};

        #[derive(Clone, Copy, Debug, Enum)]
        enum Key {
            Alpha,
            Dither,
            Fog,
            Material,
            Shade,
            Specular,
            Stencil,
            Texturing,
            VertexColor,
            Wireframe,
            ZBuffer,
        }

        type Properties = EnumMap<Key, NiLink<NiProperty>>;

        fn flatten_properties(stream: &mut NiStream, link: NiLink<NiAVObject>, mut properties: Properties) {
            let Some(object) = stream.get(link) else {
                return;
            };

            for property in &object.properties {
                let Some(object) = stream.objects.get(property.key) else {
                    continue;
                };

                let key = match object {
                    NiType::NiAlphaProperty(_) => Key::Alpha,
                    NiType::NiDitherProperty(_) => Key::Dither,
                    NiType::NiFogProperty(_) => Key::Fog,
                    NiType::NiMaterialProperty(_) => Key::Material,
                    NiType::NiShadeProperty(_) => Key::Shade,
                    NiType::NiSpecularProperty(_) => Key::Specular,
                    NiType::NiStencilProperty(_) => Key::Stencil,
                    NiType::NiTexturingProperty(_) => Key::Texturing,
                    NiType::NiVertexColorProperty(_) => Key::VertexColor,
                    NiType::NiWireframeProperty(_) => Key::Wireframe,
                    NiType::NiZBufferProperty(_) => Key::ZBuffer,
                    _ => continue,
                };

                properties[key] = *property;
            }

            if let Some(node) = stream.get_as::<_, NiNode>(link) {
                for child in node.children.clone() {
                    flatten_properties(stream, child, properties);
                }
                return;
            }

            if let Some(object) = stream.get_mut(link) {
                object.properties = properties.into_values().filter(|key| !key.is_null()).collect();
            }
        }

        for root in self.roots.clone() {
            flatten_properties(self, root.cast(), Properties::default());
        }
    }

    pub fn get_texture(&self, shape: &NiTriShape) -> String {
        for property in &shape.properties {
            let Some(tex_prop) = self.get_as::<_, NiTexturingProperty>(*property) else {
                continue;
            };
            let Some(Some(texture_map)) = tex_prop.texture_maps.first() else {
                continue;
            };
            let TextureMap::Map(base_map) = texture_map else {
                continue;
            };
            let Some(base_texture) = self.get(base_map.texture) else {
                continue;
            };
            let TextureSource::External(path) = &base_texture.source else {
                continue;
            };
            return path.to_string();
        }
        "".to_string()
    }
}

fn is_editor_marker(object: &NiAVObject) -> bool {
    object
        .name
        .starts_with_ignore_ascii_case_with_lowercase_multiple(&["editormarker", "tri editormarker"])
        .is_some()
}
