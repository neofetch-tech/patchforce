#[derive(Debug)]
struct Event {
    time: &'static str,
    title: &'static str,
    description: &'static str,
}

fn main() {
    let program = vec![
        Event {
            time: "10:00 AM",
            title: "Welcome and Opening Remarks",
            description: "Gather for a warm welcome, flag raising, and a brief history of Independence Day.",
        },
        Event {
            time: "11:00 AM",
            title: "Patriotic Music",
            description: "Enjoy a live band playing classic American songs and marching tunes.",
        },
        Event {
            time: "12:30 PM",
            title: "Family Picnic",
            description: "Share a meal with family and friends, with themed food and refreshments.",
        },
        Event {
            time: "2:00 PM",
            title: "Kids' Parade",
            description: "Children march with handmade flags and patriotic decorations.",
        },
        Event {
            time: "3:00 PM",
            title: "Community Toast",
            description: "Raise a glass to freedom and unity with a short toast from the organizers.",
        },
        Event {
            time: "4:00 PM",
            title: "Outdoor Games",
            description: "Play cornhole, sack races, and other family-friendly competitions.",
        },
        Event {
            time: "6:00 PM",
            title: "Evening Ceremony",
            description: "Listen to a guest speaker commemorating the spirit of Independence Day.",
        },
        Event {
            time: "8:00 PM",
            title: "Fireworks Finale",
            description: "Conclude the celebration with a fireworks display under the summer sky.",
        },
    ];

    println!("Independence Day Celebration Program - July 4th");
    println!("----------------------------------------------");

    for event in program {
        println!("{} - {}\n    {}\n", event.time, event.title, event.description);
    }
}
