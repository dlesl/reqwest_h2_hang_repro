# Instructions

You can use `nix-shell` to enter a shell with all dependencies

* Start nginx

    $ make start-nginx
       
* Start the stress test

    $ cargo run --release
    
* Wait for 20-30 sec (maybe longer), hit Ctrl-C, process hangs

* Setting `TIMEOUT` limits the hang
