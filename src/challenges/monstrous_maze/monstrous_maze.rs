use std::process::exit;
use crate::challenges::challenge_interface::ChallengeTrait;
use crate::challenges::monstrous_maze::{MonstrousMazeInput, MonstrousMazeOutput, stack};
use crate::challenges::monstrous_maze::cell::Cell;
use crate::challenges::monstrous_maze::information::Information;
use crate::challenges::monstrous_maze::start::Start;

pub struct MonstrousMazeChallenge {
    input: MonstrousMazeInput,
}

impl ChallengeTrait for MonstrousMazeChallenge {
    type Input = MonstrousMazeInput;
    type Output = MonstrousMazeOutput;

    fn name() -> String {
        return "MonstrousMaze".to_string();
    }

    fn new(input: Self::Input) -> Self {
        return MonstrousMazeChallenge{
            input
        }
    }

    fn solve(&self) -> Self::Output {
        // init
        let mut start = Start {
            x: 0,
            y: 0,
        };
        let mut current_cell = Cell {
            x: 0,
            y: 0,
        };

        let mut infos = Information {
            condition : Vec::from([MonstrousMazeChallenge::MONSTER, MonstrousMazeChallenge::FREE_SPACE, MonstrousMazeChallenge::EXIT, MonstrousMazeChallenge::START]),
            y_max: 0,
            x_max: 0,
            endurance: self.input.endurance,
        };

        // flag pour savoir si on a bien trouvé le caractère de début
        let mut flag = 0;
        let mut abscissa = 0;
        let mut maze_tab = Vec::new();
        let mut maze_tab_save = Vec::new();

        // lecture de la String
        for row in self.input.grid.split('\n'){
            println!("{}", row);
            let mut maze_tab_row = Vec::new(); //tableau identique au fichier
            let mut maze_tab_row_work = Vec::new(); //tableau de 1 0 -1 pour save ou on est passé
            let mut ordinate = 0; //ordonnée pour savoir ce que l'on lit

            //divise la ligne en charactères exploitable
            for charact in row.chars(){

                //attribuer un chiffre à un charactère dans le tableau de sauvegarde
                match charact{
                    START => {
                        start.x = abscissa;
                        start.y = ordinate;
                        current_cell.x = abscissa;
                        current_cell.y = ordinate;
                        maze_tab_row_work.push(2);
                        flag = 1;
                    },
                    MONSTER => {
                        maze_tab_row_work.push(4);
                    },
                    EXIT => {
                        maze_tab_row_work.push(3);
                    },
                    FREE_SPACE=> {
                        maze_tab_row_work.push(0);
                    },
                    _=> maze_tab_row_work.push(5)
                };

                maze_tab_row.push(charact);
                ordinate +=1;
            }
            maze_tab_save.push(maze_tab_row_work);
            maze_tab.push(maze_tab_row);
            abscissa +=1;
        }

        //pas de point de départ
        if flag != 1{
            exit(0);
        }

        //Def variable globale
        infos.y_max = maze_tab[0].len() ;
        infos.x_max = maze_tab.len()  ;

        let mut stack = stack::Stack::new();
        let mut path = stack::Stack::new();

        let mut turn = 0;
        let mut flag_end = false;

        while !flag_end {

            turn +=1;

            if MonstrousMazeChallenge::is_up_reachable(&current_cell, &maze_tab, &maze_tab_save, &infos){
                stack.push(Cell { x: current_cell.x, y: current_cell.y });
                path.push(MonstrousMazeChallenge::UP);
                MonstrousMazeChallenge::up(&mut current_cell, &mut maze_tab_save);
                if !MonstrousMazeChallenge::is_monster(&current_cell, &maze_tab, &mut infos){
                    flag_end = true;
                }
                if MonstrousMazeChallenge::is_success(&current_cell, &maze_tab){
                    flag_end = true;
                }
            }else{
                if MonstrousMazeChallenge::is_right_reachable(&current_cell, &maze_tab, &maze_tab_save, &infos){
                    stack.push(Cell { x: current_cell.x, y: current_cell.y });
                    path.push(MonstrousMazeChallenge::RIGHT);
                    MonstrousMazeChallenge::right(&mut current_cell, &mut maze_tab_save);
                    if !MonstrousMazeChallenge::is_monster(&current_cell, &maze_tab, &mut infos){
                        flag_end = true;
                    }
                    if MonstrousMazeChallenge::is_success(&current_cell, &maze_tab){
                        flag_end = true;
                    }
                }else{
                    if MonstrousMazeChallenge::is_down_reachable(&current_cell, &maze_tab, &maze_tab_save, &infos){
                        //println!("bas");
                        stack.push(Cell { x: current_cell.x, y: current_cell.y });
                        path.push(MonstrousMazeChallenge::DOWN);
                        MonstrousMazeChallenge::down(&mut current_cell, &mut maze_tab_save);
                        if !MonstrousMazeChallenge::is_monster(&current_cell, &maze_tab, &mut infos){
                            flag_end = true;
                        }
                        if MonstrousMazeChallenge::is_success(&current_cell, &maze_tab){
                            flag_end = true;
                        }
                    }else{
                        if MonstrousMazeChallenge::is_left_reachable(&current_cell, &maze_tab, &maze_tab_save, &infos){
                            //println!("gauche");
                            stack.push(Cell { x: current_cell.x, y: current_cell.y });
                            path.push(MonstrousMazeChallenge::LEFT);
                            MonstrousMazeChallenge::left(&mut current_cell, &mut maze_tab_save);
                            if !MonstrousMazeChallenge::is_monster(&current_cell, &maze_tab, &mut infos){
                                flag_end = true;
                            }

                            if MonstrousMazeChallenge::is_success(&current_cell, &maze_tab){
                                flag_end = true;
                            }
                        }else{
                            //println!("no direction");
                            if let Some(ancien) = stack.pop(){
                                //println!("ancien {} {} nouveau {} {}", ancien.x, ancien.y,joueur.x, joueur.y );
                                maze_tab_save[current_cell.x][current_cell.y] = 1 ;
                                current_cell.x = ancien.x;
                                current_cell.y = ancien.y;
                                maze_tab_save[current_cell.x][current_cell.y]  = 2 ;
                            }
                            path.pop();
                        }
                    }
                }
            }
        }
        for element in &stack.elements {
            maze_tab[element.x][element.y] = '#';
        }

        let str:String = MonstrousMazeChallenge::write_path_to_string(path);

        let output = MonstrousMazeOutput{
            path: str.to_string(),
        };

        return output;
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}

impl MonstrousMazeChallenge {
    const EXIT: char = 'X';
    const START: char = 'Y';
    const MONSTER: char = 'M';
    const FREE_SPACE: char = ' ';
    const UP: char = '^';
    const DOWN: char = 'v';
    const RIGHT: char = '>';
    const LEFT: char = '<';

    /// déplacement vers le haut
    fn up(current_cell: &mut Cell, maze_tab_save:&mut Vec<Vec<usize>>){
        maze_tab_save[current_cell.x][current_cell.y] = 1 ;
        current_cell.x = current_cell.x-1;
        maze_tab_save[current_cell.x][current_cell.y]  = 2 ;
    }
    fn right(current_cell: &mut Cell, maze_tab_save:&mut Vec<Vec<usize>>){
        maze_tab_save[current_cell.x][current_cell.y] = 1 ;
        current_cell.y = current_cell.y+1;
        maze_tab_save[current_cell.x][current_cell.y]  = 2 ;
    }
    fn down(current_cell: &mut Cell, maze_tab_save:&mut Vec<Vec<usize>>){
        maze_tab_save[current_cell.x][current_cell.y] = 1 ;
        current_cell.x = current_cell.x+1;
        maze_tab_save[current_cell.x][current_cell.y]  = 2 ;
    }
    fn left(current_cell: &mut Cell, maze_tab_save:&mut Vec<Vec<usize>>){
        maze_tab_save[current_cell.x][current_cell.y] = 1 ;
        current_cell.y = current_cell.y-1;
        maze_tab_save[current_cell.x][current_cell.y]  = 2 ;
    }

    fn is_up_reachable(current_cell: &Cell, maze_tab: &Vec<Vec<char>>, maze_tab_save:&Vec<Vec<usize>>, infos:&Information) -> bool {
        return if current_cell.x > 0 {
            let charact = maze_tab[current_cell.x-1][current_cell.y];
            if infos.condition.contains(&charact) && (maze_tab_save[current_cell.x-1][current_cell.y] != 1) {
                true
            }else{
                false
            }
        } else {
            false
        }
    }

    fn is_right_reachable(current_cell:&Cell, maze_tab:&Vec<Vec<char>>, maze_tab_save:&Vec<Vec<usize>>, infos:&Information) -> bool {
        return if current_cell.y+1 < infos.y_max {
            let charact = maze_tab[current_cell.x][current_cell.y+1];
            if infos.condition.contains(&charact) && (maze_tab_save[current_cell.x ][current_cell.y+ 1] != 1) {
                true
            }else{
                false
            }
        } else {
            false
        }
    }

    fn is_down_reachable(current_cell:&Cell, maze_tab:&Vec<Vec<char>>, maze_tab_save:&Vec<Vec<usize>>, infos:&Information) -> bool {
        return if current_cell.x+1 < infos.x_max {
            let charact = maze_tab[current_cell.x+1 ][current_cell.y];
            if infos.condition.contains(&charact) && (maze_tab_save[current_cell.x +1][current_cell.y] != 1) {
                true
            }else{
                false
            }
        } else {
            false
        }
    }

    fn is_left_reachable(current_cell:&Cell, maze_tab:&Vec<Vec<char>>, maze_tab_save:&Vec<Vec<usize>>, infos: &Information) -> bool {
        return if current_cell.y-1 > 0 {
            let charact = maze_tab[current_cell.x ][current_cell.y-1];
            if infos.condition.contains(&charact) && (maze_tab_save[current_cell.x][current_cell.y-1] != 1) {
                true
            }else{
                false
            }
        } else {
            false
        }
    }

    fn is_monster(current_cell:&Cell, maze_tab:&Vec<Vec<char>>, mut infos: &mut Information) -> bool{
        return if maze_tab[current_cell.x][current_cell.y] == infos.condition[0] {
            if infos.endurance > 0 {
                infos.endurance -= 1;
            }
            if infos.endurance == 0 {
                false
            } else {
                true
            }
        } else {
            true
        }
    }

    fn is_success(current_cell:&Cell, maze_tab:&Vec<Vec<char>>) -> bool {
        return if maze_tab[current_cell.x][current_cell.y] == MonstrousMazeChallenge::EXIT {
            true
        } else {
            false
        }
    }

    fn write_path_to_string(path_pile:stack::Stack<char>) -> String {
        let mut str  =  String::new();
        for element in path_pile.elements{
            str.push(element);
        }
        return str;
    }
}

#[cfg(test)]
mod tests {
    use crate::challenges::ChallengeTrait;
    use crate::challenges::monstrous_maze::{MonstrousMazeChallenge, MonstrousMazeInput};

    #[test]
    fn resolve_example_maze(){
        let input = MonstrousMazeInput {
            grid: "│Y M X│".to_string(),
            endurance : 2,
        };

        let challenge = MonstrousMazeChallenge::new(input);
        let challenge_output = challenge.solve();

        assert_eq!(challenge_output.path, ">>>>");
    }
}