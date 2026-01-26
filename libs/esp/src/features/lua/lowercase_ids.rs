use crate::*;

impl Plugin {
    pub fn lowercase_ids(&mut self) {
        for object in self.objects.iter_mut() {
            use TES3Object::*;
            match object {
                Header(object) => {
                    for (name, _) in object.masters.iter_mut() {
                        name.make_ascii_lowercase();
                    }
                }
                GameSetting(object) => {
                    object.id.make_ascii_lowercase();
                }
                GlobalVariable(object) => {
                    object.id.make_ascii_lowercase();
                }
                Class(object) => {
                    object.id.make_ascii_lowercase();
                }
                Faction(object) => {
                    object.id.make_ascii_lowercase();
                    for reaction in object.reactions.iter_mut() {
                        reaction.faction.make_ascii_lowercase();
                    }
                }
                Race(object) => {
                    object.id.make_ascii_lowercase();
                    for spell in object.spells.iter_mut() {
                        spell.make_ascii_lowercase();
                    }
                }
                Sound(object) => {
                    object.id.make_ascii_lowercase();
                    object.sound_path.make_ascii_lowercase();
                }
                SoundGen(object) => {
                    object.id.make_ascii_lowercase();
                    object.creature.make_ascii_lowercase();
                    object.sound.make_ascii_lowercase();
                }
                Skill(_object) => {}
                MagicEffect(object) => {
                    object.icon.make_ascii_lowercase();
                    object.texture.make_ascii_lowercase();
                    object.bolt_sound.make_ascii_lowercase();
                    object.cast_sound.make_ascii_lowercase();
                    object.hit_sound.make_ascii_lowercase();
                    object.area_sound.make_ascii_lowercase();
                    object.cast_visual.make_ascii_lowercase();
                    object.bolt_visual.make_ascii_lowercase();
                    object.hit_visual.make_ascii_lowercase();
                    object.area_visual.make_ascii_lowercase();
                    object.description.make_ascii_lowercase();
                }
                Script(object) => {
                    object.id.make_ascii_lowercase();
                    object.variables.make_ascii_lowercase();
                }
                Region(object) => {
                    object.id.make_ascii_lowercase();
                    object.sleep_creature.make_ascii_lowercase();
                    for (sound, _) in object.sounds.iter_mut() {
                        sound.make_ascii_lowercase();
                    }
                }
                Birthsign(object) => {
                    object.id.make_ascii_lowercase();
                    object.texture.make_ascii_lowercase();
                    for spell in object.spells.iter_mut() {
                        spell.make_ascii_lowercase();
                    }
                }
                StartScript(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                }
                LandscapeTexture(object) => {
                    object.id.make_ascii_lowercase();
                    object.file_name.make_ascii_lowercase();
                }
                Spell(object) => {
                    object.id.make_ascii_lowercase();
                }
                Static(object) => {
                    object.id.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                }
                Door(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.open_sound.make_ascii_lowercase();
                    object.close_sound.make_ascii_lowercase();
                }
                MiscItem(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                }
                Weapon(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                    object.enchanting.make_ascii_lowercase();
                }
                Container(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    for (_, item) in object.inventory.iter_mut() {
                        item.make_ascii_lowercase();
                    }
                }
                Creature(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    for (_, item) in object.inventory.iter_mut() {
                        item.make_ascii_lowercase();
                    }
                    for spell in object.spells.iter_mut() {
                        spell.make_ascii_lowercase();
                    }
                    for ai_package in object.ai_packages.iter_mut() {
                        match ai_package {
                            AiPackage::Travel(_package) => {}
                            AiPackage::Wander(_package) => {}
                            AiPackage::Escort(package) => {
                                package.target.make_ascii_lowercase();
                                // package.cell.make_ascii_lowercase();
                            }
                            AiPackage::Follow(package) => {
                                package.target.make_ascii_lowercase();
                                // package.cell.make_ascii_lowercase();
                            }
                            AiPackage::Activate(package) => {
                                package.target.make_ascii_lowercase();
                            }
                        }
                    }
                    // for destination in object.travel_destinations.iter_mut() {
                    //     destination.cell.make_ascii_lowercase();
                    // }
                    object.sound.make_ascii_lowercase();
                }
                Bodypart(object) => {
                    object.id.make_ascii_lowercase();
                    object.race.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                }
                Light(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                    object.sound.make_ascii_lowercase();
                }
                Enchanting(object) => {
                    object.id.make_ascii_lowercase();
                }
                Npc(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    for (_, item) in object.inventory.iter_mut() {
                        item.make_ascii_lowercase();
                    }
                    for spell in object.spells.iter_mut() {
                        spell.make_ascii_lowercase();
                    }
                    for ai_package in object.ai_packages.iter_mut() {
                        match ai_package {
                            AiPackage::Travel(_package) => {}
                            AiPackage::Wander(_package) => {}
                            AiPackage::Escort(package) => {
                                package.target.make_ascii_lowercase();
                                // package.cell.make_ascii_lowercase();
                            }
                            AiPackage::Follow(package) => {
                                package.target.make_ascii_lowercase();
                                // package.cell.make_ascii_lowercase();
                            }
                            AiPackage::Activate(package) => {
                                package.target.make_ascii_lowercase();
                            }
                        }
                    }
                    // for destination in object.travel_destinations.iter_mut() {
                    //     destination.cell.make_ascii_lowercase();
                    // }
                    object.race.make_ascii_lowercase();
                    object.class.make_ascii_lowercase();
                    object.faction.make_ascii_lowercase();
                    object.head.make_ascii_lowercase();
                    object.hair.make_ascii_lowercase();
                }
                Armor(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                    object.enchanting.make_ascii_lowercase();
                    for biped_object in object.biped_objects.iter_mut() {
                        biped_object.male_bodypart.make_ascii_lowercase();
                        biped_object.female_bodypart.make_ascii_lowercase();
                    }
                }
                Clothing(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                    object.enchanting.make_ascii_lowercase();
                    for biped_object in object.biped_objects.iter_mut() {
                        biped_object.male_bodypart.make_ascii_lowercase();
                        biped_object.female_bodypart.make_ascii_lowercase();
                    }
                }
                RepairItem(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                }
                Activator(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                }
                Apparatus(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                }
                Lockpick(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                }
                Probe(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                }
                Ingredient(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                }
                Book(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                    object.enchanting.make_ascii_lowercase();
                }
                Alchemy(object) => {
                    object.id.make_ascii_lowercase();
                    object.script.make_ascii_lowercase();
                    object.mesh.make_ascii_lowercase();
                    object.icon.make_ascii_lowercase();
                }
                LeveledItem(object) => {
                    object.id.make_ascii_lowercase();
                    for (item, _) in object.items.iter_mut() {
                        item.make_ascii_lowercase();
                    }
                }
                LeveledCreature(object) => {
                    object.id.make_ascii_lowercase();
                    for (creature, _) in object.creatures.iter_mut() {
                        creature.make_ascii_lowercase();
                    }
                }
                Cell(object) => {
                    // object.name.make_ascii_lowercase();
                    for (_, reference) in object.references.iter_mut() {
                        reference.id.make_ascii_lowercase();
                        if let Some(owner) = &mut reference.owner {
                            owner.make_ascii_lowercase();
                        }
                        if let Some(owner_global) = &mut reference.owner_global {
                            owner_global.make_ascii_lowercase();
                        }
                        if let Some(owner_faction) = &mut reference.owner_faction {
                            owner_faction.make_ascii_lowercase();
                        }
                        // if let Some(destination) = &mut reference.destination {
                        //     destination.cell.make_ascii_lowercase();
                        // }
                        if let Some(key) = &mut reference.key {
                            key.make_ascii_lowercase();
                        }
                        if let Some(trap) = &mut reference.trap {
                            trap.make_ascii_lowercase();
                        }
                        if let Some(soul) = &mut reference.soul {
                            soul.make_ascii_lowercase();
                        }
                    }
                }
                Landscape(_object) => {}
                PathGrid(_object) => {
                    // object.cell.make_ascii_lowercase();
                }
                Dialogue(_object) => {
                    // object.id.make_ascii_lowercase();
                }
                DialogueInfo(object) => {
                    object.id.make_ascii_lowercase();
                    object.speaker_id.make_ascii_lowercase();
                    object.speaker_race.make_ascii_lowercase();
                    object.speaker_class.make_ascii_lowercase();
                    object.speaker_faction.make_ascii_lowercase();
                    // object.speaker_cell.make_ascii_lowercase();
                    object.player_faction.make_ascii_lowercase();
                    object.sound_path.make_ascii_lowercase();
                    for filter in object.filters.iter_mut() {
                        filter.id.make_ascii_lowercase();
                    }
                }
            }
        }
    }
}
