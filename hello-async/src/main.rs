use trpl::Html;
use std::time::Duration;


async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await; 
    let response_text = response.text().await;
    Html::parse(&response_text).select_first("title").map(|title_element| title_element.inner_html())
}

async fn fut() {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;

}

async fn fut_msg() {
    let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
}

async fn fut_msg_multy () {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
}


fn main() {
    // simple spawn block
    println!("!!!!!!!!! spawn async !!!!!!!!!!!!!");
    {
        trpl::run(async {
            let handle = trpl::spawn_task(async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
            handle.await.unwrap();
        });


    }
    // future creation 
    println!("!!!!!!!!future!!!!!!!!!!!!!");
    {
        trpl::run( async {fut().await} );

    }
    println!("!!!!!!!!!future with msg!!!!!!!!!!");
    {
        trpl::run( async {fut_msg().await} );

    }

    println!("!!!!!!!future with msg multiple!!!!!!!!");
    {
        trpl::run( async {fut_msg_multy().await} );

    }


    {
        let args: Vec<String> = std::env::args().collect();
        trpl::run(async {
            let url = &args[1];
            match page_title(url).await {
                Some(title) => println!("The title for {url} was {title}"),
                None => println!("{url} had no title"),
            }
        })
    }
}
