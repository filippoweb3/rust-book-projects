fn main() {

    const DAILY_REWARDS: f64 = 25000.0; // daily rewards to active staking

    let rewards: f64 = my_rewards(DAILY_REWARDS, 66000000.0, 850000.0, "l"); // calling the function

    println!("My daily rewards from active staking are {}", rewards); // printing results

}

fn my_rewards(fix_rewards: f64, total_staked: f64, my_stake: f64, activity: &str) -> f64 {

    let rewards_per_sub: f64 = fix_rewards / total_staked; // computing the rewards per staked SUB

    let mut multiplier: Option<f64> = None;

    if activity == "low" {

        multiplier = Some(1.0);

    } else if activity == "high" {

        multiplier = Some(4.0);

    } 

    if let Some(multiplier_value) = multiplier {

        let rewards: f64 = my_stake * rewards_per_sub * multiplier_value; // computing my daily rewards

        rewards

    } else {

        // Handle the case when activity is neither "low" nor "high"
        panic!("The only options for activity are 'low' or 'high'.");

    }

}
