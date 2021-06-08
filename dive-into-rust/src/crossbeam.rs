#[cfg(test)]
mod tests {

    use crossbeam::channel;

    #[test]
    fn test_crossbeam_channel() {
        let people = vec!["Anna", "Bob", "Cody", "Dave", "Eva"];
        let (tx, rx) = channel::bounded(1);
        let (tx, rx) = (&tx, &rx);
        crossbeam::scope(|s| {
            for name in people {
                s.spawn(move |_| {
                    tx.send(name).unwrap();
                });

                if let Ok(peer) = rx.recv() {
                    println!("{} received {}'s message", name, peer);
                }

                // assert_eq!(rx.recv().unwrap(), name);
            }
        })
        .unwrap();

        // assert!(false);
    }
}
