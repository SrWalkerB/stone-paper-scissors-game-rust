use rand::Rng;

fn match_result(option_player: &str, option_ia: &str, win_option: &str, loser_option: &str) -> i32{
    if option_player == win_option && option_ia == win_option {
        return 1;
    }

    if option_player == win_option && option_ia == loser_option{
        return 2;
    }

    if option_ia == win_option && option_player == loser_option {
        return 3;
    }

    return 0;
}

fn win_display(winner_index: i32) -> i32{
    if winner_index == 1{
        println!("-> WITHOUT WIN <-");
        return 0;
    }

    if winner_index == 2 {
        println!("-> PLAYER Win <-");
        return 2;
    }

    if winner_index == 3 {
        println!("-> IA Win <-");
        return 3;
    }

    return 0;
}

fn display_options(options: &[&str; 3], player_wins: &i32, ia_wins: &i32){
    //println!("PLAYER WINS = {}", player_wins);
    //println!("IA WINS = {}", ia_wins);

    println!("---------------------");

    for index in 0..options.len(){
        println!("{} = {}", index, options[index]);
    }

    println!("---------------------");
}

fn main() {
    let options: [&str; 3] = [
        "stone", 
        "paper", 
        "scissors"
    ];
    
    
    loop {
        let mut player_wins = 0;
        let mut ia_wins = 0;
        display_options(&options, &player_wins, &ia_wins);
    
        let ia_option = rand::thread_rng().gen_range(0..options.len());
    
        let mut player_input: String = String::new();
        std::io::stdin().read_line(&mut player_input).unwrap();
    
        let option_index: usize = player_input.trim().parse().unwrap();
    
        let option_player = options[option_index];
        let option_ia = options[ia_option];
    
        println!("PLAYER = {}", options[option_index]);
        println!("IA = {}", options[ia_option]);
       
        let match_stone_scissors = match_result(option_player, option_ia, "stone", "scissors");
        let match_paper_stone = match_result(option_player, option_ia, "paper", "stone");
        let match_scissors_paper = match_result(option_player, option_ia, "scissors", "paper");
    
        let result1 = win_display(match_stone_scissors);
        let result2 = win_display(match_paper_stone);
        let result3 = win_display(match_scissors_paper);
    }


}
