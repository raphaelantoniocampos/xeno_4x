use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut current_area = Area::generate(Biome::LushForest);
    let mut specie = Species::new("Humanos", Diet::Omnivore, 70, 10);
    let mut geracao = 1;

    println!("=== BEM-VINDO AO SIMULADOR 4X ===");
    println!("Sua espécie: {} | Dieta: {:?}", specie.name, specie.diet);

    // O Game Loop principal
    loop {
        println!("\n====================================");
        println!("--- GERAÇÃO {} ---", geracao);
        println!("Bioma atual: {:?}", current_area.biome);
        println!("População disponível: {}", specie.total_population);
        println!("Estoque de Comida: {}", specie.food_stock);
        let comida_necessaria = (specie.total_population as i32) * (specie.lifespan as i32 / 10);
        println!("Comida Necessária p/ Turno: {}", comida_necessaria);
        println!("Pontos de Pesquisa: {}", specie.research_points);
        println!("====================================");

        if specie.total_population == 0 {
            println!("Sua espécie foi extinta. Fim de jogo.");
            break;
        }

        // DISPARA O EVENTO AQUI!
        disparar_evento_aleatorio(&mut specie);

        if specie.total_population == 0 {
            println!("\nA tribo não sobreviveu aos eventos recentes. Fim de jogo.");
            break;
        }

        let mut pops_restantes = specie.total_population;
        let mut alocacoes: Vec<Job> = Vec::new();

        // Fase de Alocação
        loop {
            if pops_restantes == 0 {
                println!("Todos os Pops foram alocados.");
                break;
            }

            println!("\nO que você deseja fazer na área atual?");
            println!("População livre: {}", pops_restantes);
            println!("1. Listar e Interagir com Animais");
            println!("2. Listar e Interagir com Plantas");
            println!("3. Listar e Interagir com Minerais");
            println!("4. Encerrar Alocações e Avançar Turno");

            let escolha = ler_quantidade_pops("Escolha uma opção: ");

            match escolha {
                1 => {
                    println!("\n--- ANIMAIS NA ÁREA ---");
                    for (i, animal) in current_area.animals.iter().enumerate() {
                        println!("{}. {}", i + 1, animal.description());
                    }
                    println!("0. Voltar");
                    let opt = ler_quantidade_pops("Escolha um animal ou 0 para voltar: ");
                    if opt > 0 && opt as usize <= current_area.animals.len() {
                        let idx = (opt - 1) as usize;
                        let animal = &current_area.animals[idx];

                        println!("\nAlvo: {}", animal.name);
                        println!("1. Caçar");
                        println!("2. Pesquisar");
                        println!("0. Voltar");
                        let acao = ler_quantidade_pops("Ação: ");

                        if acao == 1 || acao == 2 {
                            let qtd = ler_quantidade_pops(&format!(
                                "Quantos Pops vão realizar esta ação? (Máx {}): ",
                                pops_restantes
                            ));
                            if qtd > 0 && qtd <= pops_restantes {
                                pops_restantes -= qtd;
                                if acao == 1 {
                                    alocacoes.push(Job::Hunt {
                                        pops: qtd,
                                        target_idx: idx,
                                    });
                                } else {
                                    alocacoes.push(Job::ResearchAnimal {
                                        pops: qtd,
                                        target_idx: idx,
                                    });
                                }
                                println!("{} pops alocados.", qtd);
                            } else {
                                println!("Quantidade inválida.");
                            }
                        }
                    }
                }
                2 => {
                    println!("\n--- PLANTAS NA ÁREA ---");
                    for (i, planta) in current_area.plants.iter().enumerate() {
                        println!("{}. {}", i + 1, planta.description());
                    }
                    println!("0. Voltar");
                    let opt = ler_quantidade_pops("Escolha uma planta ou 0 para voltar: ");
                    if opt > 0 && opt as usize <= current_area.plants.len() {
                        let idx = (opt - 1) as usize;
                        let planta = &current_area.plants[idx];

                        println!("\nAlvo: {}", planta.name);
                        println!("1. Coletar");
                        println!("2. Pesquisar");
                        println!("0. Voltar");
                        let acao = ler_quantidade_pops("Ação: ");

                        if acao == 1 || acao == 2 {
                            let qtd = ler_quantidade_pops(&format!(
                                "Quantos Pops vão realizar esta ação? (Máx {}): ",
                                pops_restantes
                            ));
                            if qtd > 0 && qtd <= pops_restantes {
                                pops_restantes -= qtd;
                                if acao == 1 {
                                    alocacoes.push(Job::Forage {
                                        pops: qtd,
                                        target_idx: idx,
                                    });
                                } else {
                                    alocacoes.push(Job::ResearchPlant {
                                        pops: qtd,
                                        target_idx: idx,
                                    });
                                }
                                println!("{} pops alocados.", qtd);
                            } else {
                                println!("Quantidade inválida.");
                            }
                        }
                    }
                }
                3 => {
                    println!("\n--- MINERAIS NA ÁREA ---");
                    for (i, mineral) in current_area.minerals.iter().enumerate() {
                        println!("{}. {}", i + 1, mineral.description());
                    }
                    println!("0. Voltar");
                    let opt = ler_quantidade_pops("Escolha um mineral ou 0 para voltar: ");
                    if opt > 0 && opt as usize <= current_area.minerals.len() {
                        let idx = (opt - 1) as usize;
                        let mineral = &current_area.minerals[idx];

                        println!("\nAlvo: {}", mineral.name);
                        println!("1. Pesquisar");
                        println!("0. Voltar");
                        let acao = ler_quantidade_pops("Ação: ");

                        if acao == 1 {
                            let qtd = ler_quantidade_pops(&format!(
                                "Quantos Pops vão realizar esta ação? (Máx {}): ",
                                pops_restantes
                            ));
                            if qtd > 0 && qtd <= pops_restantes {
                                pops_restantes -= qtd;
                                alocacoes.push(Job::ResearchMineral {
                                    pops: qtd,
                                    target_idx: idx,
                                });
                                println!("{} pops alocados.", qtd);
                            } else {
                                println!("Quantidade inválida.");
                            }
                        }
                    }
                }
                4 => {
                    break;
                }
                _ => println!("Opção inválida."),
            }
        }

        if pops_restantes > 0 {
            alocacoes.push(Job::Idle(pops_restantes));
            println!("{} pops ficaram ociosos na vila.", pops_restantes);
        }

        // Processa a matemática da geração
        specie.process_generation(alocacoes, &mut current_area);

        // Pergunta se o jogador quer ir para a próxima geração
        print!("\nPressione Enter para a próxima geração ou digite 'sair' para encerrar: ");
        io::stdout().flush().unwrap();
        let mut comando = String::new();
        io::stdin().read_line(&mut comando).unwrap();

        if comando.trim().to_lowercase() == "sair" {
            println!(
                "Saindo do jogo... Sua civilização sobreviveu por {} gerações.",
                geracao
            );
            break;
        }

        geracao += 1;

        if geracao == 4 || geracao == 10 {
            println!("\n[ALERTA GLOBAL] O clã migrou para uma nova região!");
            let new_biome = if geracao == 4 {
                Biome::FrozenTundra
            } else {
                Biome::AridDesert
            };
            current_area = Area::generate(new_biome);
        }
    }
}

fn disparar_evento_aleatorio(species: &mut Species) {
    let mut rng = rand::thread_rng();

    if rng.gen_bool(0.2) {
        println!("\n⚠️ ALERTA DE EVENTO ALEATÓRIO ⚠️");
        if rng.gen_bool(0.5) {
            println!("Um ataque surpresa de uma fera errante aconteceu! (-1 Pop e -5 Comida)");
            species.total_population = species.total_population.saturating_sub(1);
            species.food_stock -= 5;
        } else {
            println!(
                "Um comerciante de terras distantes ensinou novas técnicas à tribo! (+10 Pesquisa e +10 Comida)"
            );
            species.research_points += 10;
            species.food_stock += 10;
        }
    }
}

fn ler_quantidade_pops(mensagem: &str) -> u32 {
    loop {
        print!("{}", mensagem);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");
        match input.trim().parse::<u32>() {
            Ok(numero) => return numero,
            Err(_) => println!("Entrada inválida. Digite um número inteiro."),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Diet {
    Omnivore,
    Carnivore,
    Herbivore,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Biome {
    LushForest,
    AridDesert,
    FrozenTundra,
}

#[derive(Debug, Clone)]
pub struct Animal {
    pub name: String,
    pub nutrition: u8,
    pub aggressivity: u8,
    pub strength: u8,
    pub is_nutrition_known: bool,
    pub is_aggressivity_known: bool,
    pub is_strength_known: bool,
}

impl Animal {
    pub fn new(
        name: &str,
        nutrition: u8,
        aggressivity: u8,
        strength: u8,
        reveal_nutrition: bool,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let mut a = Self {
            name: name.to_string(),
            nutrition,
            aggressivity,
            strength,
            is_nutrition_known: false,
            is_aggressivity_known: false,
            is_strength_known: false,
        };
        if reveal_nutrition {
            a.is_nutrition_known = true;
        } else {
            match rng.gen_range(0..3) {
                0 => a.is_nutrition_known = true,
                1 => a.is_aggressivity_known = true,
                _ => a.is_strength_known = true,
            }
        }
        a
    }

    pub fn description(&self) -> String {
        let nut = if self.is_nutrition_known {
            format!("Nutrição: {}", self.nutrition)
        } else {
            "Nutrição: ????".to_string()
        };
        let agg = if self.is_aggressivity_known {
            format!("Agressividade/Risco: {}", self.aggressivity)
        } else {
            "Agressividade: ????".to_string()
        };
        let str = if self.is_strength_known {
            format!("Força: {}", self.strength)
        } else {
            "Força: ????".to_string()
        };

        format!("{} [{}, {}, {}]", self.name, nut, agg, str)
    }

    pub fn reveal_random_trait(&mut self) -> bool {
        // Retorna true se algo foi revelado, false se já sabia tudo
        let mut unknown_traits = Vec::new();
        if !self.is_nutrition_known {
            unknown_traits.push(0)
        }
        if !self.is_aggressivity_known {
            unknown_traits.push(1)
        }
        if !self.is_strength_known {
            unknown_traits.push(2)
        }

        if unknown_traits.is_empty() {
            return false;
        }

        let mut rng = rand::thread_rng();
        let choice = unknown_traits[rng.gen_range(0..unknown_traits.len())];
        match choice {
            0 => self.is_nutrition_known = true,
            1 => self.is_aggressivity_known = true,
            _ => self.is_strength_known = true,
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct Plant {
    pub name: String,
    pub nutrition: u8,
    pub toxicity: u8,
    pub is_nutrition_known: bool,
    pub is_toxicity_known: bool,
}

impl Plant {
    pub fn new(name: &str, nutrition: u8, toxicity: u8, reveal_nutrition: bool) -> Self {
        let mut rng = rand::thread_rng();
        let mut p = Self {
            name: name.to_string(),
            nutrition,
            toxicity,
            is_nutrition_known: false,
            is_toxicity_known: false,
        };
        if reveal_nutrition {
            p.is_nutrition_known = true;
        } else {
            if rng.gen_bool(0.5) {
                p.is_nutrition_known = true;
            } else {
                p.is_toxicity_known = true;
            }
        }
        p
    }

    pub fn description(&self) -> String {
        let nut = if self.is_nutrition_known {
            format!("Nutrição: {}", self.nutrition)
        } else {
            "Nutrição: ????".to_string()
        };
        let tox = if self.is_toxicity_known {
            format!("Toxicidade: {}", self.toxicity)
        } else {
            "Toxicidade: ????".to_string()
        };
        format!("{} [{}, {}]", self.name, nut, tox)
    }

    pub fn reveal_random_trait(&mut self) -> bool {
        if !self.is_nutrition_known {
            self.is_nutrition_known = true;
            return true;
        }
        if !self.is_toxicity_known {
            self.is_toxicity_known = true;
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct Mineral {
    pub name: String,
    pub research_value: u8,
    pub is_value_known: bool,
}

impl Mineral {
    pub fn new(name: &str, research_value: u8) -> Self {
        Self {
            name: name.to_string(),
            research_value,
            is_value_known: false,
        }
    }
    pub fn description(&self) -> String {
        let val = if self.is_value_known {
            format!("Valor de Pesquisa: {}", self.research_value)
        } else {
            "Valor: ????".to_string()
        };
        format!("{} [{}]", self.name, val)
    }
    pub fn reveal(&mut self) -> bool {
        if !self.is_value_known {
            self.is_value_known = true;
            return true;
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct Area {
    pub biome: Biome,
    pub animals: Vec<Animal>,
    pub plants: Vec<Plant>,
    pub minerals: Vec<Mineral>,
}

impl Area {
    pub fn generate(biome: Biome) -> Self {
        let mut animals = Vec::new();
        let mut plants = Vec::new();
        let mut minerals = Vec::new();

        // O jogador SEMPRE vê a fonte de nutrição de pelo menos um recurso (Capivara ou Arbusto)
        animals.push(Animal::new("Capivara Mansa", 15, 1, 2, true));
        animals.push(Animal::new("Predador das Sombras", 40, 7, 8, false));
        animals.push(Animal::new("Mamute Encouraçado", 80, 5, 20, false));

        plants.push(Plant::new("Arbusto Frutífero", 10, 0, true));
        plants.push(Plant::new("Cogumelo Roxo Misterioso", 25, 5, false));

        minerals.push(Mineral::new("Obsidiana Brilhante", 15));
        minerals.push(Mineral::new("Rocha Porosa", 5));

        Self {
            biome,
            animals,
            plants,
            minerals,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Job {
    Hunt { pops: u32, target_idx: usize },
    Forage { pops: u32, target_idx: usize },
    ResearchAnimal { pops: u32, target_idx: usize },
    ResearchPlant { pops: u32, target_idx: usize },
    ResearchMineral { pops: u32, target_idx: usize },
    Idle(u32),
}

#[derive(Debug)]
pub struct Species {
    pub name: String,
    pub diet: Diet,
    pub lifespan: u8,
    pub total_population: u32,
    pub food_stock: i32,
    pub research_points: u32,
}

impl Species {
    pub fn new(name: &str, diet: Diet, lifespan: u8, initial_pop: u32) -> Self {
        Self {
            name: name.to_string(),
            diet,
            lifespan: lifespan,
            total_population: initial_pop,
            food_stock: 15,
            research_points: 0,
        }
    }

    pub fn process_generation(&mut self, allocations: Vec<Job>, current_area: &mut Area) {
        let mut food_gathered = 0;
        let mut research_gained = 0;
        let mut deaths_this_turn = 0;
        let mut rng = rand::thread_rng();

        for job in allocations {
            match job {
                Job::Hunt { pops, target_idx } => {
                    let animal = &current_area.animals[target_idx];
                    println!(
                        "Relatório: {} Pops tentaram caçar '{}'...",
                        pops, animal.name
                    );

                    // Risco calculado: Força + Agressividade. Mais pops diluem o risco.
                    let base_risk = (animal.strength as u32) + (animal.aggressivity as u32);
                    let pop_strength = pops * 3; // Força inicial inata de cada humano

                    // Maior quantidade de pops aumenta drasticamente a chance de sucesso
                    let sucess_probability =
                        (pop_strength as f32 / (pop_strength as f32 + base_risk as f32)).min(0.95);

                    if rng.gen_bool(sucess_probability as f64) {
                        let food = (animal.nutrition as u32) * pops; // Rendimento mais generoso
                        println!(
                            "> VITÓRIA! A caça foi bem-sucedida. Rendimento: +{} Comida.",
                            food
                        );
                        food_gathered += food;
                    } else {
                        // Risco de acordo com quantidade de pops e agressividade do animal
                        let deaths = rng.gen_range(0..=(animal.aggressivity as u32)).min(pops);
                        println!("> FRACASSO... A caça deu errado.");
                        if deaths > 0 {
                            println!(
                                ">> FATALIDADE! O animal revidou e {} caçadores perderam a vida.",
                                deaths
                            );
                            deaths_this_turn += deaths;
                        } else {
                            println!(">> A caça fugiu sem causar ferimentos fatais.");
                        }
                    }
                }
                Job::Forage { pops, target_idx } => {
                    let plant = &current_area.plants[target_idx];
                    println!("Relatório: {} Pops coletaram '{}'...", pops, plant.name);

                    let base_food = (plant.nutrition as u32) * pops;
                    food_gathered += base_food;

                    if plant.toxicity > 0 {
                        let tox_damage = (plant.toxicity as u32) * (pops / 2);
                        println!("> CUIDADO! A planta exalava esporos tóxicos.");
                        if rng.gen_bool(0.3) {
                            let deaths = 1.min(pops);
                            deaths_this_turn += deaths;
                            println!(
                                ">> FATALIDADE Tóxica: {} pops morreram envenenados.",
                                deaths
                            );
                        } else {
                            food_gathered = food_gathered.saturating_sub(tox_damage);
                            println!(">> As toxinas estragaram grande parte do que foi colhido.");
                        }
                    } else {
                        println!("> Coleta segura e farta (+{} Comida).", base_food);
                    }
                }
                Job::ResearchAnimal { pops, target_idx } => {
                    let animal = &mut current_area.animals[target_idx];
                    println!(
                        "Relatório: {} Pops pesquisaram o comportamento de '{}'...",
                        pops, animal.name
                    );

                    // Pesquisar animais agressivos também tem risco de morte
                    let research_risk_base = (animal.aggressivity as f32 / 2.0).ceil() as u32;
                    // Se o animal for perigoso e poucos batedores (pops), a chance de morrer aumenta.
                    let fail_prob = (research_risk_base as f32 / (pops as f32 * 2.0)).min(0.5); // Máx 50% chance

                    if rng.gen_bool(fail_prob.into()) && research_risk_base > 0 {
                        let deaths = 1.min(pops);
                        deaths_this_turn += deaths;
                        println!(
                            "> TRAGÉDIA NA PESQUISA! O animal surpreendeu os pesquisadores. (-{} Pop)",
                            deaths
                        );
                    } else {
                        if animal.reveal_random_trait() {
                            println!(
                                "> SUCESSO! Nova característica deduzida através da observação."
                            );
                        } else {
                            println!(
                                "> Nada de novo a aprender, mas anotações comportamentais valeram pontos."
                            );
                        }
                    }
                    research_gained += pops * 2;
                }
                Job::ResearchPlant { pops, target_idx } => {
                    let plant = &mut current_area.plants[target_idx];
                    println!("Relatório: {} Pops catalogaram '{}'...", pops, plant.name);
                    if plant.reveal_random_trait() {
                        println!(
                            "> SUCESSO! Propriedades botânicas antes ocultas foram reveladas."
                        );
                    }
                    research_gained += pops * 2;
                }
                Job::ResearchMineral { pops, target_idx } => {
                    let mineral = &mut current_area.minerals[target_idx];
                    println!(
                        "Relatório: {} Pops inspecionaram '{}'...",
                        pops, mineral.name
                    );
                    if mineral.reveal() {
                        println!("> SUCESSO! Confirmamos a utilidade do mineral.");
                        research_gained += (mineral.research_value as u32) + (pops * 2);
                    } else {
                        research_gained += pops;
                    }
                }
                Job::Idle(_) => {}
            }
        }

        self.total_population = self.total_population.saturating_sub(deaths_this_turn);

        let food_consumed = (self.total_population as i32) * (self.lifespan as i32 / 10);
        let food_surplus = (food_gathered as i32) - food_consumed;
        self.food_stock = food_surplus;
        self.research_points += research_gained;

        println!("\n--- RESULTADOS DO FIM DE TURNO ---");
        if self.food_stock < 0 {
            let starvation = self.food_stock.abs() as u32;
            let deaths = starvation.min(self.total_population);
            self.total_population -= deaths;
            self.food_stock = 0;
            println!(
                "Fome severa... faltou {} comida para {} bocas e eles pereceram de inanição.",
                starvation, deaths,
            );
        } else if self.food_stock >= (self.total_population * 2) as i32 && self.total_population > 0
        {
            let growth = (self.total_population as f32 * 0.15).ceil() as u32;
            self.total_population += growth;
            self.food_stock -= growth as i32;
            println!(
                "Tempo de fartura! Os estoques permitiram um boom populacional, crescemos em {}.",
                growth
            );
        } else {
            println!("A população sobreviveu a este ciclo sem perdas por fome.");
        }
    }
}
