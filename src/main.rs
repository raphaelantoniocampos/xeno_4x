use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord { pub x: usize, pub y: usize }

#[derive(Resource)]
pub struct WorldMap {
    pub areas: Vec<Vec<Area>>,
    pub width: usize,
    pub height: usize,
}

impl WorldMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut areas = Vec::new();
        for _y in 0..height {
            let mut row = Vec::new();
            for _x in 0..width {
                let r = rand::thread_rng().gen_range(0..4);
                let biome = match r {
                    0 => Biome::LushForest, 1 => Biome::AridDesert,
                    2 => Biome::Savana, _ => Biome::FrozenTundra,
                };
                row.push(Area::generate(biome));
            }
            areas.push(row);
        }
        Self { areas, width, height }
    }
    pub fn get_area_mut(&mut self, coord: Coord) -> Option<&mut Area> {
        if coord.x < self.width && coord.y < self.height { Some(&mut self.areas[coord.y][coord.x]) } else { None }
    }
    pub fn get_area(&self, coord: Coord) -> Option<&Area> {
        if coord.x < self.width && coord.y < self.height { Some(&self.areas[coord.y][coord.x]) } else { None }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Diet { Omnivore, Carnivore, Herbivore }

#[derive(Debug, Clone, PartialEq)]
pub enum Biome { LushForest, AridDesert, FrozenTundra, ToxicForest, Montains, Savana, TropicalIsle }

#[derive(Debug, Clone)]
pub struct Animal {
    pub name: String, pub diet: Diet, pub nutrition: u8, pub strength: u8,
    pub aggressivity: u8, pub population: f32, pub reproduction_rate: f32,
    pub is_quantity_known: bool, pub is_domesticated: bool, pub domestication_progress: f32,
}

impl Animal {
    pub fn new(name: &str, diet: Diet, pop: f32, reprod: f32, nut: u8, strg: u8, agg: u8) -> Self {
        Self { name: name.to_string(), diet, population: pop, reproduction_rate: reprod, nutrition: nut, strength: strg, aggressivity: agg, is_quantity_known: false, is_domesticated: false, domestication_progress: 0.0 }
    }
}

#[derive(Debug, Clone)]
pub struct Plant {
    pub name: String, pub nutrition: u8, pub toxicity: u8, pub population: f32,
    pub reproduction_rate: f32, pub is_quantity_known: bool, pub is_domesticated: bool, pub domestication_progress: f32,
}

impl Plant {
    pub fn new(name: &str, pop: f32, reprod: f32, nut: u8, tox: u8) -> Self {
        Self { name: name.to_string(), population: pop, reproduction_rate: reprod, nutrition: nut, toxicity: tox, is_quantity_known: false, is_domesticated: false, domestication_progress: 0.0 }
    }
}

#[derive(Debug, Clone)]
pub struct Area { pub biome: Biome, pub animals: Vec<Animal>, pub plants: Vec<Plant> }

impl Area {
    pub fn generate(biome: Biome) -> Self {
        let mut animals = Vec::new();
        let mut plants = Vec::new();
        let mut rng = rand::thread_rng();
        match biome {
            Biome::LushForest => {
                animals.push(Animal::new("Capivara Mansa", Diet::Herbivore, rng.gen_range(100.0..300.0), 0.20, 10, 1, 0)); 
                animals.push(Animal::new("Lince Feroz", Diet::Carnivore, rng.gen_range(10.0..30.0), 0.05, 15, 5, 5));
                animals.push(Animal::new("Javali Irritadiço", Diet::Omnivore, rng.gen_range(40.0..80.0), 0.12, 20, 6, 7));
                plants.push(Plant::new("Arbusto Frutífero", rng.gen_range(500.0..1000.0), 0.15, 5, 0));
                plants.push(Plant::new("Mandioca", rng.gen_range(200.0..400.0), 0.10, 15, 0));
            }
            Biome::AridDesert => {
                animals.push(Animal::new("Lagarto Corredor", Diet::Carnivore, rng.gen_range(50.0..100.0), 0.10, 5, 2, 1));
                plants.push(Plant::new("Cacto Suculento", rng.gen_range(100.0..300.0), 0.05, 8, 0));
            }
            Biome::FrozenTundra => {
                animals.push(Animal::new("Mamute Felpudo", Diet::Herbivore, rng.gen_range(5.0..15.0), 0.02, 50, 8, 3));
                animals.push(Animal::new("Lobo do Inverno", Diet::Carnivore, rng.gen_range(10.0..25.0), 0.08, 15, 4, 8));
                plants.push(Plant::new("Líquen Amargo", rng.gen_range(300.0..800.0), 0.15, 2, 0));
            }
            Biome::Savana => {
                animals.push(Animal::new("Trabuco Gigante", Diet::Herbivore, rng.gen_range(30.0..60.0), 0.08, 35, 7, 3));
                animals.push(Animal::new("Leão da Areia", Diet::Carnivore, rng.gen_range(5.0..10.0), 0.04, 25, 8, 10));
                plants.push(Plant::new("Grama Alta", rng.gen_range(800.0..1500.0), 0.20, 1, 0));
            }
            _ => {
                plants.push(Plant::new("Mato Comum", rng.gen_range(100.0..200.0), 0.1, 2, 0));
                animals.push(Animal::new("Rato", Diet::Omnivore, 100.0, 0.5, 1, 0, 0));
            }
        }
        Self { biome, animals, plants }
    }
}

#[derive(Debug, Clone)]
pub enum Job {
    Hunt { pops: u32, target_idx: usize, coord: Coord },
    Forage { pops: u32, target_idx: usize, coord: Coord },
    ResearchArea { pops: u32, coord: Coord },
    ResearchAnimal { pops: u32, target_idx: usize, coord: Coord },
    ResearchPlant { pops: u32, target_idx: usize, coord: Coord },
    Herd { pops: u32, target_idx: usize, coord: Coord },
    Farm { pops: u32, target_idx: usize, coord: Coord },
    Idle(u32),
}

#[derive(Resource, Debug)]
pub struct Species {
    pub name: String, pub diet: Diet, pub lifespan: u8, pub total_population: u32,
    pub food_stock: i32, pub research_points: u32, pub location: Coord,
}

impl Species {
    pub fn new(name: &str, diet: Diet, lifespan: u8, initial_pop: u32) -> Self {
        Self { name: name.to_string(), diet, lifespan, total_population: initial_pop, food_stock: 50, research_points: 0, location: Coord { x: 3, y: 3 } }
    }

    pub fn process_year(&mut self, world: &mut WorldMap, allocations: &mut Vec<Job>) -> (u32, bool, Option<String>) {
        let mut food_gathered = 0;
        let mut meat_gathered = 0; 
        let mut interrupt = None;
        let mut rng = rand::thread_rng();

        for row in world.areas.iter_mut() {
            for area in row.iter_mut() {
                let mut plant_pop_total: f32 = area.plants.iter().map(|p| p.population).sum();
                let herb_pop_total: f32 = area.animals.iter().filter(|a| !a.is_domesticated && (a.diet == Diet::Herbivore || a.diet == Diet::Omnivore)).map(|a| a.population).sum();
                
                for animal in area.animals.iter_mut() {
                    if animal.is_domesticated && animal.diet == Diet::Carnivore {
                        let demanda_carne = animal.population * 2.0;
                        if (meat_gathered as f32) >= demanda_carne {
                            meat_gathered -= demanda_carne as u32; 
                        } else {
                            interrupt = Some(format!("Os '{}' domesticados não receberam carne suficiente e estão devorando as capivaras / população! Morte de Pops e Fuga de animais!", animal.name));
                            animal.is_domesticated = false;
                            let mortes_fuga = rng.gen_range(1..=3).min(self.total_population);
                            self.total_population -= mortes_fuga;
                            continue;
                        }
                    }

                    if animal.population <= 0.0 || animal.is_domesticated { continue; } 
                    
                    if animal.diet == Diet::Herbivore || animal.diet == Diet::Omnivore {
                        let consumo_planta = animal.population;
                        if plant_pop_total >= consumo_planta { 
                            plant_pop_total -= consumo_planta; 
                            animal.population += animal.population * animal.reproduction_rate; 
                        } else {
                            animal.population -= animal.population * rng.gen_range(0.1..0.3);
                        }
                    } else if animal.diet == Diet::Carnivore {
                        let consumo_carne = animal.population * 2.0;
                        if herb_pop_total >= consumo_carne {
                             animal.population += animal.population * animal.reproduction_rate;
                        } else {
                             animal.population -= animal.population * rng.gen_range(0.15..0.4);
                        }
                    }
                }
                
                for plant in area.plants.iter_mut() {
                    if plant.population <= 0.0 { continue; }
                    plant.population += plant.population * plant.reproduction_rate;
                }
            }
        }

        for job in allocations.iter_mut() {
            match job {
                Job::Hunt { pops, target_idx, coord } => {
                    if *pops == 0 { continue; }
                    let area = world.get_area_mut(*coord).unwrap();
                    let animal = &mut area.animals[*target_idx];
                    if animal.population < 1.0 {
                        interrupt = Some(format!("Extinção de '{}' na área ({},{}). Caçadores pararam.", animal.name, coord.x, coord.y));
                        *pops = 0; continue; 
                    }
                    let abate_real = (*pops as f32).min(animal.population);
                    animal.population -= abate_real;
                    meat_gathered += (abate_real as u32) * (animal.nutrition as u32);
                }
                Job::Forage { pops, target_idx, coord } => {
                    if *pops == 0 { continue; }
                    let area = world.get_area_mut(*coord).unwrap();
                    let plant = &mut area.plants[*target_idx];
                    if plant.population < 1.0 { 
                        interrupt = Some(format!("Planta '{}' extinta em ({},{}). Coletores ociosos.", plant.name, coord.x, coord.y));
                        *pops = 0; continue; 
                    }
                    let coleta_real = (*pops as f32 * 2.0).min(plant.population);
                    plant.population -= coleta_real;
                    food_gathered += (coleta_real as u32) * (plant.nutrition as u32);
                }
                Job::ResearchArea { pops, coord } => {
                    if *pops == 0 { continue; }
                    let area = world.get_area_mut(*coord).unwrap();
                    for a in area.animals.iter_mut() { a.is_quantity_known = true; }
                    for p in area.plants.iter_mut() { p.is_quantity_known = true; }
                    interrupt = Some(format!("Mapeamento Exato da Área ({}, {}) concluído.", coord.x, coord.y));
                    *pops = 0; 
                }
                Job::ResearchAnimal { pops, target_idx, coord } => {
                    if *pops == 0 { continue; }
                    let area = world.get_area_mut(*coord).unwrap();
                    let animal = &mut area.animals[*target_idx];
                    if !animal.is_domesticated {
                        animal.domestication_progress += (0.01 * (*pops as f32)) / (1.0 + animal.aggressivity as f32);
                        if animal.domestication_progress >= 1.0 {
                            animal.domestication_progress = 1.0;
                            animal.is_domesticated = true;
                            interrupt = Some(format!("DOMESTICAMOS o(a) '{}'!!!", animal.name));
                            let (p, i, c) = (*pops, *target_idx, *coord);
                            *job = Job::Herd { pops: p, target_idx: i, coord: c };
                        }
                    }
                }
                Job::ResearchPlant { pops, target_idx, coord } => {
                    if *pops == 0 { continue; }
                    let area = world.get_area_mut(*coord).unwrap();
                    let plant = &mut area.plants[*target_idx];
                    if !plant.is_domesticated {
                        plant.domestication_progress += 0.01 * (*pops as f32); 
                        if plant.domestication_progress >= 1.0 {
                            plant.domestication_progress = 1.0;
                            plant.is_domesticated = true;
                            interrupt = Some(format!("Fazendas orgânicas de '{}' criadas!!!", plant.name));
                            let (p, i, c) = (*pops, *target_idx, *coord);
                            *job = Job::Farm { pops: p, target_idx: i, coord: c };
                        }
                    }
                }
                Job::Herd { pops, target_idx, coord } => {
                    if *pops == 0 { continue; }
                    let area = world.get_area_mut(*coord).unwrap();
                    let animal = &mut area.animals[*target_idx];
                    if animal.population < 1.0 { continue; }
                    animal.population += animal.population * (animal.reproduction_rate * 0.5); 
                    food_gathered += *pops * (animal.nutrition as u32 / 2);
                }
                Job::Farm { pops, target_idx, coord } => {
                    if *pops == 0 { continue; }
                    let area = world.get_area_mut(*coord).unwrap();
                    let plant = &mut area.plants[*target_idx];
                    if plant.population < 1.0 { plant.population = 1.0; } 
                    plant.population += plant.population * plant.reproduction_rate; 
                    food_gathered += *pops * (plant.nutrition as u32);
                }
                Job::Idle(_) => {}
            }
        }

        let mut deaths = 0;
        let mut starve_flag = false;
        let comida_necessaria_ano = self.total_population as i32; 
        
        food_gathered += meat_gathered;
        if self.food_stock > 0 { self.food_stock = (self.food_stock as f32 * 0.90).floor() as i32; }
        self.food_stock += food_gathered as i32 - comida_necessaria_ano;

        if self.food_stock < 0 {
            deaths = (self.food_stock.abs() as u32).min(self.total_population);
            self.total_population -= deaths;
            self.food_stock = 0;
            starve_flag = true;
        } else if self.food_stock >= (self.total_population * 2) as i32 && self.total_population > 0 {
            let growth = (self.total_population as f32 * 0.03).ceil() as u32; 
            self.total_population += growth;
            self.food_stock -= growth as i32 * 4; 
        }

        (deaths, starve_flag, interrupt)
    }
}

// -------------------------------------------------------------
// BEVY ECS / EGUI SYSTEMS 
// -------------------------------------------------------------

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Simulation,
}

#[derive(Resource)]
pub struct SimTimer(Timer);

#[derive(Resource)]
pub struct GameSession {
    pub current_year: u8,
    pub generation: u32,
    pub deaths_turn: u32,
    pub alocacoes: Vec<Job>,
    pub starvation_triggered: bool,
    pub populacao_trabalhadora: u32,
    pub pops_start: u32,
}

#[derive(Resource, Default)]
pub struct UiState {
    pub selected_coord: Option<Coord>,
    pub interrupt_msgs: Vec<String>,
    pub sliders_anim: Vec<u32>,
    pub sliders_pl: Vec<u32>, 
    pub sliders_area: u32,
}

fn estimar_quantidade(pop: f32, is_known: bool) -> String {
    if is_known { format!("{:.0}", pop) }
    else {
        if pop <= 0.0 { "Nenhum".to_string() }
        else if pop < 10.0 { "Escasso".to_string() }
        else if pop < 50.0 { "Razoável".to_string() }
        else if pop < 150.0 { "Abundante".to_string() }
        else { "Maciço".to_string() }
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(WorldMap::new(6, 6));
    let start_species = Species::new("Humanos", Diet::Omnivore, 60, 20);
    commands.insert_resource(GameSession {
        current_year: 1,
        generation: 1,
        deaths_turn: 0,
        alocacoes: Vec::new(),
        starvation_triggered: false,
        populacao_trabalhadora: start_species.total_population,
        pops_start: start_species.total_population,
    });
    commands.insert_resource(start_species);
    commands.insert_resource(UiState::default());
    commands.insert_resource(SimTimer(Timer::from_seconds(0.1, TimerMode::Repeating))); // 10 years a second
}

// GUI for the Left and Right panels
fn ui_system(
    mut contexts: EguiContexts,
    mut state: ResMut<NextState<GameState>>,
    mut species: ResMut<Species>,
    mut world: ResMut<WorldMap>,
    mut gs: ResMut<GameSession>,
    mut ui_state: ResMut<UiState>
) {
    let ctx = contexts.ctx_mut();
    
    // Calcula alocados
    gs.alocacoes.retain(|job| match job {
        Job::Idle(_) => false,
        Job::Hunt { pops, .. } | Job::Forage { pops, .. } | Job::ResearchArea { pops, .. } |
        Job::ResearchAnimal { pops, .. } | Job::ResearchPlant { pops, .. } |
        Job::Herd { pops, .. } | Job::Farm { pops, .. } => *pops > 0,
    });
    let mut alocados: u32 = gs.alocacoes.iter().map(|j| match j {
        Job::Idle(_) => 0,
        Job::Hunt { pops, .. } | Job::Forage { pops, .. } | Job::ResearchArea { pops, .. } |
        Job::ResearchAnimal { pops, .. } | Job::ResearchPlant { pops, .. } |
        Job::Herd { pops, .. } | Job::Farm { pops, .. } => *pops,
    }).sum();
    
    if gs.populacao_trabalhadora > species.total_population {
        gs.populacao_trabalhadora = species.total_population;
    }
    if alocados > gs.populacao_trabalhadora {
        gs.alocacoes.clear();
        alocados = 0;
        ui_state.interrupt_msgs.push("A Tribo reduziu abaixo da frota de alocados. Cancelei trabalhos!".to_string());
    }
    let pops_livres = gs.populacao_trabalhadora.saturating_sub(alocados);

    // PANEL DIREITO (STATUS)
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        ui.heading("Tribo");
        ui.label(format!("Raça: {}", species.name));
        ui.label(format!("Estoque Food: {}", species.food_stock));
        ui.label(format!("População Total: {}", species.total_population));
        ui.label(format!("Workers Livres: {}", pops_livres));
        ui.separator();
        ui.label(format!("Localização: ({}, {})", species.location.x, species.location.y));
        ui.separator();
        ui.heading(format!("Geração {}", gs.generation));
        ui.label(format!("Ano: {} / {}", gs.current_year, species.lifespan));
        
        ui.add_space(20.0);
        if ui.button("▶ AVANÇAR TEMPO").clicked() {
            state.set(GameState::Simulation);
        }

        ui.add_space(20.0);
        if !ui_state.interrupt_msgs.is_empty() {
             ui.heading(egui::RichText::new("ALERTAS").color(egui::Color32::RED));
             for msg in &ui_state.interrupt_msgs {
                 ui.label(msg);
             }
             if ui.button("Limpar Alertas").clicked() {
                 ui_state.interrupt_msgs.clear();
             }
        }
    });

    // PANEL ESQUERDO (AÇÕES DE ÁREA)
    egui::SidePanel::right("right_panel").default_width(320.0).show(ctx, |ui| {
        if let Some(coord) = ui_state.selected_coord {
            let is_home = coord == species.location;
            ui.heading(format!("Inspetor ({}, {})", coord.x, coord.y));
            if ui.button("Migrar Tribo para cá").clicked() {
                 species.location = coord;
                 ui_state.interrupt_msgs.push(format!("Tribo migrou para ({},{})", coord.x, coord.y));
            }
            ui.separator();
            ui.label("Pesquisar Área Integral");
             ui.add(egui::Slider::new(&mut ui_state.sliders_area, 0..=pops_livres).text("Batedores"));
             if ui.button("Designar Batedores").clicked() && ui_state.sliders_area > 0 {
                 gs.alocacoes.push(Job::ResearchArea { pops: ui_state.sliders_area, coord });
                 ui_state.sliders_area = 0;
             }
            
            ui.separator();
            let area = world.get_area_mut(coord).unwrap();
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Animais Locais");
                if ui_state.sliders_anim.len() != area.animals.len() * 3 {
                    ui_state.sliders_anim.resize(area.animals.len() * 3, 0);
                }
                for (i, animal) in area.animals.iter_mut().enumerate() {
                    let dom = if animal.is_domesticated { "🐶 Dom." } else { "🐺 Selvagem" };
                    ui.group(|ui| {
                        ui.label(format!("{} [{}]", animal.name, dom));
                        ui.label(format!("Pop: {} | Agressivo: {}", estimar_quantidade(animal.population, animal.is_quantity_known), animal.aggressivity));
                        ui.label(format!("Progresso Adestramento: {:.1}%", animal.domestication_progress * 100.0));
                        
                        // Action: Caçar
                        ui.horizontal(|ui| {
                            ui.add(egui::Slider::new(&mut ui_state.sliders_anim[i*3], 0..=pops_livres));
                            if ui.button("Caçar").clicked() && ui_state.sliders_anim[i*3] > 0 {
                                gs.alocacoes.push(Job::Hunt { pops: ui_state.sliders_anim[i*3], target_idx: i, coord });
                                ui_state.sliders_anim[i*3] = 0;
                            }
                        });
                        // Action: Research
                        if !animal.is_domesticated {
                           ui.horizontal(|ui| {
                               ui.add(egui::Slider::new(&mut ui_state.sliders_anim[i*3+1], 0..=pops_livres));
                               if ui.button("Pesquisar/Domar").clicked() && ui_state.sliders_anim[i*3+1] > 0 {
                                   gs.alocacoes.push(Job::ResearchAnimal { pops: ui_state.sliders_anim[i*3+1], target_idx: i, coord });
                                   ui_state.sliders_anim[i*3+1] = 0;
                               }
                           });
                        } else if is_home { // Pastoreio vs Abate
                           ui.horizontal(|ui| {
                               ui.add(egui::Slider::new(&mut ui_state.sliders_anim[i*3+2], 0..=pops_livres));
                               if ui.button("Pastorear (Herd)").clicked() && ui_state.sliders_anim[i*3+2] > 0 {
                                   gs.alocacoes.push(Job::Herd { pops: ui_state.sliders_anim[i*3+2], target_idx: i, coord });
                                   ui_state.sliders_anim[i*3+2] = 0;
                               }
                           });
                           if ui.button("Abater Rebanho!").clicked() && animal.population >= 1.0 {
                               let carne = (animal.population as i32) * (animal.nutrition as i32);
                               species.food_stock += carne;
                               animal.population = 0.0;
                               ui_state.interrupt_msgs.push(format!("Abate de {} gerou +{} carne.", animal.name, carne));
                           }
                        }
                    });
                }
                
                ui.separator();
                ui.heading("Vegetação Local");
                if ui_state.sliders_pl.len() != area.plants.len() * 3 {
                    ui_state.sliders_pl.resize(area.plants.len() * 3, 0);
                }
                for (i, plant) in area.plants.iter_mut().enumerate() {
                    let dom = if plant.is_domesticated { "🌱 Fazenda" } else { "🌿 Mato" };
                    ui.group(|ui| {
                        ui.label(format!("{} [{}]", plant.name, dom));
                        ui.label(format!("Pop: {}", estimar_quantidade(plant.population, plant.is_quantity_known)));
                        ui.label(format!("Progresso Agro: {:.1}%", plant.domestication_progress * 100.0));
                        
                        ui.horizontal(|ui| {
                            ui.add(egui::Slider::new(&mut ui_state.sliders_pl[i*3], 0..=pops_livres));
                            if ui.button("Forragear").clicked() && ui_state.sliders_pl[i*3] > 0 {
                                gs.alocacoes.push(Job::Forage { pops: ui_state.sliders_pl[i*3], target_idx: i, coord });
                                ui_state.sliders_pl[i*3] = 0;
                            }
                        });
                        if !plant.is_domesticated {
                           ui.horizontal(|ui| {
                               ui.add(egui::Slider::new(&mut ui_state.sliders_pl[i*3+1], 0..=pops_livres));
                               if ui.button("Pesquisar Sementes").clicked() && ui_state.sliders_pl[i*3+1] > 0 {
                                   gs.alocacoes.push(Job::ResearchPlant { pops: ui_state.sliders_pl[i*3+1], target_idx: i, coord });
                                   ui_state.sliders_pl[i*3+1] = 0;
                               }
                           });
                        } else if is_home { 
                           ui.horizontal(|ui| {
                               ui.add(egui::Slider::new(&mut ui_state.sliders_pl[i*3+2], 0..=pops_livres));
                               if ui.button("Fazenda (Farm)").clicked() && ui_state.sliders_pl[i*3+2] > 0 {
                                   gs.alocacoes.push(Job::Farm { pops: ui_state.sliders_pl[i*3+2], target_idx: i, coord });
                                   ui_state.sliders_pl[i*3+2] = 0;
                               }
                           });
                        }
                    });
                }
            });
        }
    });

    // MAPA CENTRAL
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("World Map 6x6");
        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new("map_grid").spacing(egui::vec2(5.0, 5.0)).show(ui, |ui| {
                for y in 0..world.height {
                    for x in 0..world.width {
                        let is_home = species.location == Coord{x, y};
                        let selected = ui_state.selected_coord == Some(Coord{x,y});
                        let area = world.get_area(Coord{x,y}).unwrap();
                        
                        let color = match area.biome {
                            Biome::LushForest => egui::Color32::from_rgb(34, 139, 34),
                            Biome::AridDesert => egui::Color32::from_rgb(210, 180, 140),
                            Biome::Savana => egui::Color32::from_rgb(244, 164, 96),
                            Biome::FrozenTundra => egui::Color32::from_rgb(175, 238, 238),
                            _ => egui::Color32::GRAY,
                        };
                        
                        let label = if is_home { "🛖 HOME" } else { "      " };
                        let btn = egui::Button::new(label).fill(color).min_size(egui::vec2(80.0, 80.0));
                        let btn = if selected { btn.stroke(egui::Stroke::new(3.0, egui::Color32::WHITE)) } else { btn };
                        
                        if ui.add(btn).clicked() {
                            ui_state.selected_coord = Some(Coord{x,y});
                            ui_state.sliders_anim.clear(); // Reset sliders
                            ui_state.sliders_pl.clear();
                        }
                    }
                    ui.end_row();
                }
            });
        });
    });
}

fn tick_simulation(
    time: Res<Time>,
    mut timer: ResMut<SimTimer>,
    mut state: ResMut<NextState<GameState>>,
    mut species: ResMut<Species>,
    mut world: ResMut<WorldMap>,
    mut gs: ResMut<GameSession>,
    mut ui_state: ResMut<UiState>
) {
    if timer.0.tick(time.delta()).just_finished() {
        if gs.current_year > species.lifespan || species.total_population == 0 {
            // Fim de geração ou extinção humana!
            ui_state.interrupt_msgs.push(format!("-- GERAÇÃO {} ENCERRADA -- População: {}", gs.generation, species.total_population));
            gs.generation += 1;
            gs.current_year = 1;
            gs.populacao_trabalhadora = species.total_population;
            state.set(GameState::Menu);
            return;
        }

        let (mortes_este_ano, fame, interrupt_msg) = species.process_year(&mut world, &mut gs.alocacoes);
        gs.deaths_turn += mortes_este_ano;
        
        if fame && !gs.starvation_triggered {
            ui_state.interrupt_msgs.push(format!("[Ano {}] Fome devastadora na tribo!", gs.current_year));
            gs.starvation_triggered = true;
            state.set(GameState::Menu);
        }

        if let Some(msg) = interrupt_msg {
            ui_state.interrupt_msgs.push(format!("[Ano {}] INTERRUPT: {}", gs.current_year, msg));
            state.set(GameState::Menu);
        }

        gs.current_year += 1;
    }
}

// Simulacao Tela de UI minima
fn sim_ui(mut contexts: EguiContexts, gs: Res<GameSession>, species: Res<Species>) {
    egui::Window::new("MÁQUINA DO TEMPO...").anchor(egui::Align2::CENTER_CENTER, [0.0,0.0]).collapsible(false).show(contexts.ctx_mut(), |ui| {
        ui.heading("Simulando Anos...");
        ui.label(format!("Ano Atual: {} / {}", gs.current_year, species.lifespan));
        ui.add(egui::ProgressBar::new(gs.current_year as f32 / species.lifespan as f32).animate(true));
        ui.label("As engrenagens do ecossistema estão girando. Aguarde.");
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { title: "Xeno 4X".into(), resolution: (1024., 768.).into(), ..default() }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system.run_if(in_state(GameState::Menu)))
        .add_systems(Update, (tick_simulation, sim_ui).run_if(in_state(GameState::Simulation)))
        .run();
}
