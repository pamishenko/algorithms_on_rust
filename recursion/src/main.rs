use rand::Rng;

static mut COUNT: u32 = 0;

#[derive(Debug)]
struct Box {
    offspring: Option<Vec<Box>>,
    key: bool,
}

impl Box {
    unsafe fn generate_offspring() -> Option<Vec<Box>> {
        let mut rng = rand::thread_rng();
        let offspring_count = rng.gen_range(0..3);
        let mut offsprings: Option<Vec<Box>> = None;
        if offspring_count > 0 {
            let mut offspring_vec = Vec::with_capacity(offspring_count);
            for _ in 0..offspring_count {
                let offspring = Box {
                    offspring: Box::generate_offspring(),
                    key: rng.gen_bool(0.1),
                };
                offspring_vec.push(offspring);
                COUNT = COUNT + 1;
            }
            offsprings = Some(offspring_vec);
        }
        offsprings
    }
}

fn find_key(vec_boxes:  &Vec<Box>) {
    for v in vec_boxes.iter(){
        if v.key == true {
            println!("Found!!");
        } else if let Some(offspring) = &v.offspring{
                find_key(offspring);
            };
        }
    }

fn main() {
    let my_box = unsafe { Box::generate_offspring() };
    if let Some(v) = my_box {
        for s in &v {
            println!("{:#?}", s);
            find_key(&v);
        }
    } else {
        println!("Vector is empty.");
    }
    unsafe { println!("COUNT = {}", COUNT); }
}
