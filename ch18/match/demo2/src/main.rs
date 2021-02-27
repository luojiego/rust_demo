fn main() {
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        // while let 
        while let Some(top) = stack.pop() {
            println!("pop {}", top);
        }

        println!("while let end");
    }

    {
        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    {
        let point = (3, 5);
        print_coordinates(&point);
    }

    {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        let x = 5;
        match x {
            1..=5 => println!("one through five"),
            6..=9 => println!("six through nine"),
            _ => println!("something else"),
        }
    }

    {
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7};
        let Point{ x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
        let Point{x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        match p {
            Point {x, y: 0} => println!("On the x axis at {}", x),
            Point {x: 0, y} => println!("On the y axis at {}", y),
            Point {x, y} => println!("On neither axis: ({}, {})", x, y),
        }
    }

    {
        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructer.")
            }
            Message::Move {x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y)
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => println!(
                "Change the color to red {}, green {} and blue {}",
                r, g, b
            )
        }
    }

    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move {x: i32, y: i32},
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
                "Change the color to red {}, green {} and blue {}",
                r, g, b
            ),
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
                "Change the color to hue {}, saturation {} and blue {}",
                h, s, v
            ),
            _ => (),
        }
    }

    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }
    }

    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}",
                    first, third, fifth)
            }
        }
    }

    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point {x: 1, y: 0, z: 0};
        match origin {
            Point {x, ..} => println!("x is {}", x),
        }
    }

    {
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }
    }

    {
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("less than five: {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }

    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {}", n),
            _ => println!("Default case, x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {}", x, y);
    }

    {
        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    {
        enum Message {
            Hello {id : i32},
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),
            Message::Hello {
                id: 10..=12
            } => {
                println!("Found an id in another range")
            }
            Message::Hello {id} => println!("Found some other id: {}", id),
        }
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y)
}