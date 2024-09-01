use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    const N: usize = 10;
    const PAUSE: Duration = Duration::from_millis(500);

    // Создаем два канала
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let tx2_clone = tx2.clone(); 

    // Запускаем поток для вычисления квадратов
    thread::spawn(move || {
        for num in rx1 {
            let squared = num * num;
            tx2.send(squared).unwrap();
        }
    });

    // Запускаем поток для вывода результатов
    thread::spawn(move || {
        for squared in rx2 {
            println!("{}", squared);
        }
    });

    // Заполняем первый канал числами
    for i in 1..=N {
        tx1.send(i).unwrap();
        thread::sleep(PAUSE);
    }

    // Ждем, пока все потоки завершат работу
    drop(tx1);
    drop(tx2_clone);
}