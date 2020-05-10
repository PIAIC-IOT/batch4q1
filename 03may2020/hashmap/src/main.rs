use std::collections::HashMap;

fn main() {
    let data = ("Abid".to_string(),34,50000);
    //index      0                 1   2
    let age = [22,35,34,23,67];
    //index       0   1   2   3   4

    let mut fruit = HashMap::new();
    fruit.insert("Summer","Mango");
    fruit.insert("Winter","Orange");
    fruit.insert("Autumn","Apple");
    fruit.insert("Spring","Strawberry");
    fruit.entry("Spring").or_insert("Banana");
    //              key,    value
    //            index,    value
    println!("Complete Hashmap Fruit : {:#?}",fruit);
    let index_of_fruit = fruit.get("Spring");
    println!("index of Fruit : {:?} \n",index_of_fruit);

    let mut team = HashMap::new();
    team.insert("Pakistan",200);
    team.insert("Pakistan",400);
    team.insert("Australia",230);
    team.insert("China",230);
    println!("Complete Hashmap team : {:#?}",team);
    let index_of_team = team.get("Pakistan");
    println!("index of team : {:?} \n",index_of_team);

    let keys = vec![22,44];
    let values = vec!["Germany","China","Pakistan","New Zealand"];
    let mut rank :HashMap<_,_> = keys.into_iter().zip(values.into_iter()).collect();
    println!("Complete Hashmap team : {:#?}",rank);
    let index_of_rank = rank.get(&100);
    println!("index of rank : {:?} \n",index_of_rank);


    let text = "samajh samajh kar samajh ko samjho";
    // let text = "Faisalabad is beutiful city. 
    //             Kohinoor is elite class area of Faisalabad
    //             Old Name of Faisalabad is lyallpur";
    let mut word_count = HashMap::new();
    for mydata in text.split_whitespace(){
        let temp = word_count.entry(mydata).or_insert(0);
        * temp +=1;
    }
    println!("{:#?}",word_count);

}