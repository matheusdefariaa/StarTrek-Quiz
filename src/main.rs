/* Quiz sobre Star trek */
/* Esse quiz aborda as series Star Trek
A serie, Deep space nine. Voyager, Enterprise */

use std::{io,process, process::Command, thread, time::{self}};

fn main() {
    clear_screen();
    start();

    loop{
        clear_screen();
        main_screen();
        let mut opc:String = String::new();
        io::stdin()
        .read_line(&mut opc)
        .expect("Erro ao ler/*  */");

        let opc:i32 = match opc.trim().parse::<i32>() {
            Ok(n) if n >= 1 && n <= 4 => n,
            Ok(_) =>  {
                println!("  --------------------------------------");
                println!("    Desculpe, sua entrada não é válida  ");
                println!("  --------------------------------------");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            },
            Err(_) => {
                println!("  --------------------------------------");
                println!("    Desculpe, sua entrada não é válida  ");
                println!("  --------------------------------------");
                thread::sleep(time::Duration::from_secs(2));
                continue;
            }
        };

        match  opc {
            1 => start_game(),
            2 => help(),
            3 => the_way_back_home(),
            4 => exit(),
            _ => println!("Error"),
        };

    }

}


fn start() {
    println!("\n
        ███████ ████████  █████  ██████      ████████ ██████  ███████ ██   ██                ██████  ██    ██ ██ ███████ 
        ██         ██    ██   ██ ██   ██        ██    ██   ██ ██      ██  ██                ██    ██ ██    ██ ██    ███  
        ███████    ██    ███████ ██████         ██    ██████  █████   █████       █████     ██    ██ ██    ██ ██   ███   
             ██    ██    ██   ██ ██   ██        ██    ██   ██ ██      ██  ██                ██ ▄▄ ██ ██    ██ ██  ███    
        ███████    ██    ██   ██ ██   ██        ██    ██   ██ ███████ ██   ██                ██████   ██████  ██ ███████ 
                                                                                                ▀▀                       
                                                                                                                        
    ");
    thread::sleep(time::Duration::from_secs(3));

}


fn clear_screen() {
    Command::new("clear")
    .status()
    .expect("Error");
}

fn help() {
    let mut cont:String = String::new();
    clear_screen();
    println!(r"
                    _   _      _       
                    | | | | ___| |_ __  
                    | |_| |/ _ \ | '_ \ 
                    |  _  |  __/ | |_) |
                    |_| |_|\___|_| .__/ 
                                |_|    
");
    println!("-----------                                       -----------");
    println!(" PERGUNTAS                                         PONTUAÇÃO ");
    println!("-----------                                       -----------");
                                        
    println!("                          
    O jogo contem 10 perguntas.                        Cada pergunta vale 1 ponto.

    As perguntas incluem:                              Acertos:

    -------------------                                  >= 9 -> Um verdadeiro tripulante 🖖
        Star Trek                                        >= 7 -> Fã 🏅
    -------------------                                  >= 5 -> Deixei passando na TV 📺
   * Série original                                       < 5 -> Confundiu com Star Wars ? 🙃
   * The Next Generation
   * Deep space nine
   * Voyager
   * Enterprise\n");

    println!("\n\tAperte Enter para continuar");
    io::stdin()
    .read_line(&mut cont)
    .expect("Error");

}


fn main_screen() {
    println!(r"                             
        _ __ ___   ___ _ __  _   _ 
        | '_ ` _ \ / _ \ '_ \| | | |
        | | | | | |  __/ | | | |_| |
        |_| |_| |_|\___|_| |_|\__,_|
    ");
    println!("    ╭╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╮");
    println!("    |      (1) - 🆂🆃🅰🆁🆃 🅶🅰🅼🅴            |");
    println!("    |      (2) - 🅷🅴🅻🅿                  |");
    println!("    |      (3) - 🆃🅷🅴 🆆🅰🆈 🅱🅰🅲🅺 🅷🅾🅼🅴     |");
    println!("    |      (4) - 🅴🆇🅸🆃                  |");
    println!("    ╰╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╶╯");

}

fn start_game() {
    let mut name:String = String::new();

    clear_screen();
    println!("🅳🅰🆃🅰 🅴🆂🆃🅴🅻🅰🆁 2024.8");

    println!("    -------------------------------");
    println!("      Digite seu nome tripulante ⌨️");
    println!("    -------------------------------");

    io::stdin()
    .read_line(&mut name)
    .expect("Error reading");
    clear_screen();

    println!("🅳🅰🆃🅰 🅴🆂🆃🅴🅻🅰🆁 2024.8");
    println!(" Tripulante {name}
    Oficial de ciências da nave estelar Enterprise foi designado a participar de um torneio de perguntas em Vulcano.
    Sera uma batalha difícil, mas toda a frota estelar vai está torcendo por você.");

    thread::sleep(time::Duration::from_secs(6));

    questions();
    }

fn exit() {
    clear_screen();
    println!(r"
       ____                       _   ____                 
     / ___|   ___     ___     __| | | __ )   _   _    ___ 
    | |  _   / _ \   / _ \   / _` | |  _ \  | | | |  / _ \
    | |_| | | (_) | | (_) | | (_| | | |_) | | |_| | |  __/
     \____|  \___/   \___/   \__,_| |____/   \__, |  \___|
                                            |___/        
     ");
    process::exit(0)
}

fn questions() {
    let mut contagem  = 0;
    let resp = vec![3,2,3,1,2,4,1,1,4,2];
    let mut tec:String = String::new();

    let questions = vec!["
        -------------------------------------------------------------------------------------------------------

                                                    Questão 1 🚀

        Qual é o nome do primeiro ser humano a inventar motor de dobra, no ano de 2063,
        chamando a atenção dos Vulcanos, proporcionando o primeiro contato da humanidade com uma raça alienígena.

        --------------------------------------------------------------------------------------------------------",

        "   
        -------------------------------------------------------------------------------------------------------

                                                    Questão 2 🚀

        Qual é o nome da nave da frota estelar perdida no quadrante delta, localizado a 70 mil anos-luz da Terra.

        -------------------------------------------------------------------------------------------------------",
        
        "
        -------------------------------------------------------------------------------------------------------

                                                    Questão 3 🚀

        Em Deep space nine a estação espacial Bajoriana foi dominado por uma raça alienígena.
        Qual raça era essa. 
        -------------------------------------------------------------------------------------------------------",

        "
        -------------------------------------------------------------------------------------------------------

                                                    Questão 4 🚀

        Não é Will. Ele é Will, mas... você sabe o que quero dizer - Beverly Crusher.
        No episódio Second Chances descobrimos que um acidente de teletransporte no ano de 2361 criou dois tenentes Riker,
        geneticamente indistinguíveis um do outro, com personalidade e memórias idênticas até o ponto da duplicação.
        Uma das duplicatas continuou a ser conhecida como William Riker.
        O outro escolheu usar seu nome do meio e ser conhecido como. 
        
        -------------------------------------------------------------------------------------------------------",

        "
        -------------------------------------------------------------------------------------------------------

                                                    Questão 5 🚀

        The Cage é o episódio piloto original da série de ficção científica Star Trek e da franquia resultante.
        Foi completado no início de 1965, porém não foi ao ar em sua forma completa até outubro de 1988.
        Qual era o nome do capitão na nave estelar Enterprise.

        -------------------------------------------------------------------------------------------------------",
        "
        -------------------------------------------------------------------------------------------------------

                                                    Questão 6 🚀

        Tenente Worf interpretado pelo ator Michael Dorn esteve presente em duas series da franquia Star Trek,
        Star Trek Next Generations e Deep Space Nine.
        Mesmo tendo crescido entre humanos Worf pertence a outra raça.
        Qual raça era.

        -------------------------------------------------------------------------------------------------------",
        "
        -------------------------------------------------------------------------------------------------------

                                                    Questão 7 🚀

        Leonard Nimoy foi um ator importantíssimo para Star Trek, o ator protagonizou o personagem Spock.
        Nimoy foi o responsável pela criação da famosa saudação vucana.

        -------------------------------------------------------------------------------------------------------",
        "
        -------------------------------------------------------------------------------------------------------

                                                    Questão 8 🚀

        O que diz a primeira diretriz da frota estelar.

        -------------------------------------------------------------------------------------------------------",
        "
        -------------------------------------------------------------------------------------------------------

                                                    Questão 9 🚀

        Qual o nome do melhor amigo de Jake Sisko em Deep Space Nine.

        -------------------------------------------------------------------------------------------------------",
        "
        -------------------------------------------------------------------------------------------------------

                                                    Questão 10 🚀

        Em Deep Space Nine o Dr Julian Bashir esconde um segredo.
        Qual segredo era.

        -------------------------------------------------------------------------------------------------------", 
        ];

    let opc_resp = vec![
    "(1) Christopher Pike  (2) Montgomery Scott  (3) Zefram Cochrane  (4) Malcolm  Reed",
    "(1) USS Enterprise  (2) USS Voyager (3) USS Defiant (4) USS Constellation",
    "(1) Vulcanos  (2) Ferengis  (3) Cardassianos  (4) Romulanos",
    "(1) Thomas Riker  (2) Kyle Riker  (3) Erik Riker  (4) Wesley Riker",
    "(1) James T. Kirk  (2) Christopher Pike  (3) Jonathan Archer  (4) Charles Tucker",
    "(1) Borg  (2) Espécie 8472  (3) Breen  (4) Klingons",
    "(1) Vida longa e próspera  (2) Ao infinito e além  (3) Vida longa e alegre.  (4) Vida próspera e alegre",
    "(1) Não interferência  (2) Ajudar sempre  (3) Humanos primeiro  (4) Primeiro os outros",
    "(1) Odo  (2) Rom  (3) Quark  (4) Nog",
    "(1) Adotado  (2) Geneticamente alterado  (3) Não humano  (4) Não é medico",

    ];

    for (i,e) in questions.iter().enumerate() {
        println!("{e}");
        println!("\n\t{:?}",opc_resp[i]);


    loop{
        let mut opc_question: String = String::new();
        io::stdin()
        .read_line(&mut opc_question)
        .expect("Error reading");

        let opc_question: i32 = match opc_question.trim().parse::<i32>() {
            Ok(n) if n >= 1 && n <= 4 => n,
            Ok(_) =>  {
                println!("  --------------------------------------");
                println!("    Desculpe, sua entrada não é válida  ");
                println!("        Digite uma opção valida: ");
                println!("  --------------------------------------");
                continue;
            },
            Err(_) => {
                println!("  --------------------------------------");
                println!("    Desculpe, sua entrada não é válida  ");
                println!("        Digite uma opção valida: ");
                println!("  --------------------------------------");

                continue;
            }
        };

        if opc_question == resp[i] {
            println!("\t---------------------------------\n\t\t𝙍𝙚𝙨𝙥𝙤𝙨𝙩𝙖 𝙘𝙚𝙧𝙩𝙖 ✅\n\t---------------------------------");
            contagem += 1;
            break;
        }

        else {
            println!("\t---------------------------------\n\t\t𝙍𝙚𝙨𝙥𝙤𝙨𝙩𝙖 𝙚𝙧𝙧𝙖𝙙𝙖 ❌\n\t---------------------------------");
            break;
        }

    }
}

    match contagem {
        9..=10 => println!("\n\tUm verdadeiro tripulante 🖖 - Nota {contagem}/10"),
        7..=8 => println!("\n\tDeixei passando na TV 📺 - Nota {contagem}/10"),
        5..=6 => println!("\n\tFã 🏅- Nota {contagem}/10"),
        _ => println!("\n\tConfundiu com Star Wars ? 🙃 - Nota {contagem}/10")
    }

    println!("\n\tAperte Enter para continuar");
    io::stdin()
    .read_line(&mut tec)
    .expect("Erro ao ler");

}

fn the_way_back_home() {
    clear_screen();
    println!(r"
            .-------------..--.                   _.--------.___
        |  ==========> ||   )             _.--'              `--._
        `-------------'`--'           .-'   \                    `-._
            /--.     /               .'       \.--------.__           `.
            /---'    |      _____.--.'`-.__     \  ___      `-._         `.
        .-'   ____.-------'      .'     __`-._.--'__ `-.       `.         \
        .'  |.-'    ___________.--'------'  _ |  .-/[_],  `.   |   `.        \
        |---' | .--'  _____________[|___   | || |]]]/ \|   |   | .-----.  .--.|
        |---. | `--.___________    [|      |_|| |]]]\_/|   |   | `-----'  `--'|
        `.  |`-.____           `--.------.__  |_ `-\[_]`  .'   |   .'        /
        `-.       `-------._____ `.     __.-' `--.___.-'      _.'         /
            \---.    |           `--`..-'       /         __.-'          .'
            \--'     \               `.       /`--------'            _.'
        .-------------..--.           `-._  /                   _.-'
        |  ==========> ||   )              `--._          ___.--'
        `-------------'`--'                    `--------'                                                                           
                                                                                                                        
    ");
    thread::sleep(time::Duration::from_secs(2));
    clear_screen();

    println!(r"
                                                        ________
                                        __.------'--------`---.___
                                    _.--'  _.-' /==================`--.___
                            __.-'     .'    /                          `--._
            .--------------' --------'=======================================
        ___.-'-----------..__                 .'`-._________.---------'
        '--|  ==========<=|| [`.______________/
            `-------------'`---'             /
                    `._____              /
                            `------------'                                                             

    ");
    thread::sleep(time::Duration::from_secs(2));
    clear_screen();

    println!(r"
                        ___
                __.----'---`----.__
            _.--'=========O=========`--._
        _.-'              o              `-._
        =====================================
        .----.__`---._________.---'__.----.
        |/__\|------/___ _ ___\------|/__\|
        `----'     |   .' `.   |     `----'
                    \  `._.'  /
                    `-------'
    ");

    thread::sleep(time::Duration::from_secs(2));
    clear_screen();

    println!(r"
          ____ _____      _______
         / __//_  _/ ,/| /____   )
        ( (    / / ,/  |   __ ) /
         \ \  / /,/ _  |  / // /
      ____) )/ //,-' `.| / / \ \
     /_____//_///     ||/_/   \ \_ ______ ______     ______  __   __
                               \_//_  __//___   )   / ____/ / /  / /
                                   / /    __ ) /   / /_    / / /'/'
                                  / /    / // /   / ___|  / //'/'
                                 / /    / / \ \  / /____ / / \ \
                                /_/    /_/   \ \/______//_/   \_\
                                              \/
    ");

    thread::sleep(time::Duration::from_secs(2));


}