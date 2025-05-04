use crate::prelude::*;

mod avoidnode;
mod brickniextradata;
mod bsmirrorednode;
mod enums;
mod niaccumulator;
mod nialphaaccumulator;
mod nialphacontroller;
mod nialphaproperty;
mod niambientlight;
mod niautonormalparticles;
mod niautonormalparticlesdata;
mod niavobject;
mod nibillboardnode;
mod nibltsource;
mod nibound;
mod niboundingvolume;
mod niboxbv;
mod nibsanimationmanager;
mod nibsanimationnode;
mod nibsparraycontroller;
mod nibsparticlenode;
mod nibspnode;
mod nicamera;
mod niclusteraccumulator;
mod nicollisionswitch;
mod nicolordata;
mod nicolorkey;
mod nidirectionallight;
mod niditherproperty;
mod nidx8renderer;
mod nidynamiceffect;
mod niemittermodifier;
mod niextradata;
mod niflipcontroller;
mod nifloatcontroller;
mod nifloatdata;
mod nifloatkey;
mod nifltanimationnode;
mod nifogproperty;
mod nigeometry;
mod nigeometrydata;
mod nigeommorphercontroller;
mod nigravity;
mod nikeyframecontroller;
mod nikeyframedata;
mod nikeyframemanager;
mod nilight;
mod nilightcolorcontroller;
mod nilines;
mod nilinesdata;
mod nilink;
mod nilodnode;
mod nilookatcontroller;
mod nimaterialcolorcontroller;
mod nimaterialproperty;
mod nimorphdata;
mod nimorphercontroller;
mod ninode;
mod niobject;
mod niobjectnet;
mod nipalette;
mod niparticlebomb;
mod niparticlecollider;
mod niparticlecolormodifier;
mod niparticlegrowfade;
mod niparticlemodifier;
mod niparticlerotation;
mod niparticles;
mod niparticlesdata;
mod niparticlesystemcontroller;
mod nipathcontroller;
mod niperparticledata;
mod nipixeldata;
mod nipixelformat;
mod niplanarcollider;
mod nipointlight;
mod niposdata;
mod niposkey;
mod niproperty;
mod nirenderedcubemap;
mod nirenderedtexture;
mod nirenderer;
mod nirollcontroller;
mod nirotatingparticles;
mod nirotatingparticlesdata;
mod nirotdata;
mod nirotkey;
mod niscreenpolygon;
mod nisequence;
mod nisequencestreamhelper;
mod nishadeproperty;
mod niskindata;
mod niskininstance;
mod niskinpartition;
mod nisortadjustnode;
mod nisourcetexture;
mod nispecularproperty;
mod nispherebv;
mod nisphericalcollider;
mod nispotlight;
mod nistencilproperty;
mod nistream;
mod nistringextradata;
mod niswitchnode;
mod nitextkeyextradata;
mod nitexture;
mod nitextureeffect;
mod nitexturingproperty;
mod nitimecontroller;
mod nitribasedgeom;
mod nitribasedgeomdata;
mod nitrishape;
mod nitrishapedata;
mod nitrishapedynamicdata;
mod nitristrips;
mod nitristripsdata;
mod niunionbv;
mod niuvcontroller;
mod niuvdata;
mod nivertexcolorproperty;
mod nivertweightsextradata;
mod niviscontroller;
mod nivisdata;
mod niwireframeproperty;
mod nizbufferproperty;
mod rootcollisionnode;
mod tes3objectextradata;

pub use avoidnode::*;
pub use brickniextradata::*;
pub use bsmirrorednode::*;
pub use enums::*;
pub use niaccumulator::*;
pub use nialphaaccumulator::*;
pub use nialphacontroller::*;
pub use nialphaproperty::*;
pub use niambientlight::*;
pub use niautonormalparticles::*;
pub use niautonormalparticlesdata::*;
pub use niavobject::*;
pub use nibillboardnode::*;
pub use nibltsource::*;
pub use nibound::*;
pub use niboundingvolume::*;
pub use niboxbv::*;
pub use nibsanimationmanager::*;
pub use nibsanimationnode::*;
pub use nibsparraycontroller::*;
pub use nibsparticlenode::*;
pub use nibspnode::*;
pub use nicamera::*;
pub use niclusteraccumulator::*;
pub use nicollisionswitch::*;
pub use nicolordata::*;
pub use nicolorkey::*;
pub use nidirectionallight::*;
pub use niditherproperty::*;
pub use nidx8renderer::*;
pub use nidynamiceffect::*;
pub use niemittermodifier::*;
pub use niextradata::*;
pub use niflipcontroller::*;
pub use nifloatcontroller::*;
pub use nifloatdata::*;
pub use nifloatkey::*;
pub use nifltanimationnode::*;
pub use nifogproperty::*;
pub use nigeometry::*;
pub use nigeometrydata::*;
pub use nigeommorphercontroller::*;
pub use nigravity::*;
pub use nikeyframecontroller::*;
pub use nikeyframedata::*;
pub use nikeyframemanager::*;
pub use nilight::*;
pub use nilightcolorcontroller::*;
pub use nilines::*;
pub use nilinesdata::*;
pub use nilink::*;
pub use nilodnode::*;
pub use nilookatcontroller::*;
pub use nimaterialcolorcontroller::*;
pub use nimaterialproperty::*;
pub use nimorphdata::*;
pub use nimorphercontroller::*;
pub use ninode::*;
pub use niobject::*;
pub use niobjectnet::*;
pub use nipalette::*;
pub use niparticlebomb::*;
pub use niparticlecollider::*;
pub use niparticlecolormodifier::*;
pub use niparticlegrowfade::*;
pub use niparticlemodifier::*;
pub use niparticlerotation::*;
pub use niparticles::*;
pub use niparticlesdata::*;
pub use niparticlesystemcontroller::*;
pub use nipathcontroller::*;
pub use niperparticledata::*;
pub use nipixeldata::*;
pub use nipixelformat::*;
pub use niplanarcollider::*;
pub use nipointlight::*;
pub use niposdata::*;
pub use niposkey::*;
pub use niproperty::*;
pub use nirenderedcubemap::*;
pub use nirenderedtexture::*;
pub use nirenderer::*;
pub use nirollcontroller::*;
pub use nirotatingparticles::*;
pub use nirotatingparticlesdata::*;
pub use nirotdata::*;
pub use nirotkey::*;
pub use niscreenpolygon::*;
pub use nisequence::*;
pub use nisequencestreamhelper::*;
pub use nishadeproperty::*;
pub use niskindata::*;
pub use niskininstance::*;
pub use niskinpartition::*;
pub use nisortadjustnode::*;
pub use nisourcetexture::*;
pub use nispecularproperty::*;
pub use nispherebv::*;
pub use nisphericalcollider::*;
pub use nispotlight::*;
pub use nistencilproperty::*;
pub use nistream::*;
pub use nistringextradata::*;
pub use niswitchnode::*;
pub use nitextkeyextradata::*;
pub use nitexture::*;
pub use nitextureeffect::*;
pub use nitexturingproperty::*;
pub use nitimecontroller::*;
pub use nitribasedgeom::*;
pub use nitribasedgeomdata::*;
pub use nitrishape::*;
pub use nitrishapedata::*;
pub use nitrishapedynamicdata::*;
pub use nitristrips::*;
pub use nitristripsdata::*;
pub use niunionbv::*;
pub use niuvcontroller::*;
pub use niuvdata::*;
pub use nivertexcolorproperty::*;
pub use nivertweightsextradata::*;
pub use niviscontroller::*;
pub use nivisdata::*;
pub use niwireframeproperty::*;
pub use nizbufferproperty::*;
pub use rootcollisionnode::*;
pub use tes3objectextradata::*;

#[derive(NiType, Clone, Debug, From, PartialEq)]
pub enum NiType {
    AvoidNode(AvoidNode),
    BrickNiExtraData(BrickNiExtraData),
    BSMirroredNode(BSMirroredNode),
    NiAccumulator(NiAccumulator),
    NiAlphaAccumulator(NiAlphaAccumulator),
    NiAlphaController(NiAlphaController),
    NiAlphaProperty(NiAlphaProperty),
    NiAmbientLight(NiAmbientLight),
    NiAutoNormalParticles(NiAutoNormalParticles),
    NiAutoNormalParticlesData(NiAutoNormalParticlesData),
    NiAVObject(NiAVObject),
    NiBillboardNode(NiBillboardNode),
    NiBltSource(NiBltSource),
    NiBSAnimationManager(NiBSAnimationManager),
    NiBSAnimationNode(NiBSAnimationNode),
    NiBSPArrayController(NiBSPArrayController),
    NiBSParticleNode(NiBSParticleNode),
    NiBSPNode(NiBSPNode),
    NiCamera(NiCamera),
    NiClusterAccumulator(NiClusterAccumulator),
    NiCollisionSwitch(NiCollisionSwitch),
    NiColorData(NiColorData),
    NiDirectionalLight(NiDirectionalLight),
    NiDitherProperty(NiDitherProperty),
    NiDX8Renderer(NiDX8Renderer),
    NiDynamicEffect(NiDynamicEffect),
    NiEmitterModifier(NiEmitterModifier),
    NiExtraData(NiExtraData),
    NiFlipController(NiFlipController),
    NiFloatController(NiFloatController),
    NiFloatData(NiFloatData),
    NiFltAnimationNode(NiFltAnimationNode),
    NiFogProperty(NiFogProperty),
    NiGeometry(NiGeometry),
    NiGeometryData(NiGeometryData),
    NiGeomMorpherController(NiGeomMorpherController),
    NiGravity(NiGravity),
    NiKeyframeController(NiKeyframeController),
    NiKeyframeData(NiKeyframeData),
    NiKeyframeManager(NiKeyframeManager),
    NiLight(NiLight),
    NiLightColorController(NiLightColorController),
    NiLines(NiLines),
    NiLinesData(NiLinesData),
    NiLODNode(NiLODNode),
    NiLookAtController(NiLookAtController),
    NiMaterialColorController(NiMaterialColorController),
    NiMaterialProperty(NiMaterialProperty),
    NiMorphData(NiMorphData),
    NiMorpherController(NiMorpherController),
    NiNode(NiNode),
    NiObject(NiObject),
    NiObjectNET(NiObjectNET),
    NiPalette(NiPalette),
    NiParticleBomb(NiParticleBomb),
    NiParticleCollider(NiParticleCollider),
    NiParticleColorModifier(NiParticleColorModifier),
    NiParticleGrowFade(NiParticleGrowFade),
    NiParticleModifier(NiParticleModifier),
    NiParticleRotation(NiParticleRotation),
    NiParticles(NiParticles),
    NiParticlesData(NiParticlesData),
    NiParticleSystemController(NiParticleSystemController),
    NiPathController(NiPathController),
    NiPixelData(NiPixelData),
    NiPlanarCollider(NiPlanarCollider),
    NiPointLight(NiPointLight),
    NiPosData(NiPosData),
    NiProperty(NiProperty),
    NiRenderedCubeMap(NiRenderedCubeMap),
    NiRenderedTexture(NiRenderedTexture),
    NiRenderer(NiRenderer),
    NiRollController(NiRollController),
    NiRotatingParticles(NiRotatingParticles),
    NiRotatingParticlesData(NiRotatingParticlesData),
    NiRotData(NiRotData),
    NiScreenPolygon(NiScreenPolygon),
    NiSequenceStreamHelper(NiSequenceStreamHelper),
    NiShadeProperty(NiShadeProperty),
    NiSkinData(NiSkinData),
    NiSkinInstance(NiSkinInstance),
    NiSkinPartition(NiSkinPartition),
    NiSortAdjustNode(NiSortAdjustNode),
    NiSourceTexture(NiSourceTexture),
    NiSpecularProperty(NiSpecularProperty),
    NiSphericalCollider(NiSphericalCollider),
    NiSpotLight(NiSpotLight),
    NiStencilProperty(NiStencilProperty),
    NiStringExtraData(NiStringExtraData),
    NiSwitchNode(NiSwitchNode),
    NiTextKeyExtraData(NiTextKeyExtraData),
    NiTexture(NiTexture),
    NiTextureEffect(NiTextureEffect),
    NiTexturingProperty(NiTexturingProperty),
    NiTimeController(NiTimeController),
    NiTriBasedGeom(NiTriBasedGeom),
    NiTriBasedGeomData(NiTriBasedGeomData),
    NiTriShape(NiTriShape),
    NiTriShapeData(NiTriShapeData),
    NiTriShapeDynamicData(NiTriShapeDynamicData),
    NiTriStrips(NiTriStrips),
    NiTriStripsData(NiTriStripsData),
    NiUVController(NiUVController),
    NiUVData(NiUVData),
    NiVertexColorProperty(NiVertexColorProperty),
    NiVertWeightsExtraData(NiVertWeightsExtraData),
    NiVisController(NiVisController),
    NiVisData(NiVisData),
    NiWireframeProperty(NiWireframeProperty),
    NiZBufferProperty(NiZBufferProperty),
    RootCollisionNode(RootCollisionNode),
    TES3ObjectExtraData(TES3ObjectExtraData),
}

impl NiType {
    #[inline]
    pub fn is_instance_of<T>(&self) -> bool
    where
        for<'a> &'a Self: TryInto<&'a T>,
    {
        self.try_into().is_ok()
    }
}
