HomeStation
-----------

- Bored of constantly looking at your phone to check time and weather? 

- Looking for a way to enhance quality of your life?

- Have a spare Raspberry Pi and HD44780?

Build yourself a small weather-home-station!

You'd still have to constantly check time and weather, but at least now on a professional-grade HD44780 screen.

# Photos

A photo of HomeStation in real life - at a window sill of mine:

![](/readme/home-station-irl.jpg)

The application in the photo is running with Polish language enabled - it says `Overall state: :-)))`.

# Features

- Shows current date (amazing!),

- Shows current time (even more amazing!),

- Heck, it even shows current temperature, pressure and humidity!

- Moreover - shows the PM2.5 and PM10 levels, and overall air quality (easily extendable, if you want to fiddle with it).

Thanks to [Airly](https://airly.eu)!

# Compiling

To compile, you'll need:

- Rust (tested on 1.26.0),
- Cross-compiling environment (https://github.com/japaric/rust-cross),
- Cross-compiled OpenSSL (unless you plan to compile on a Raspberry Pi itself).

The compiling process is as simple as:

```
$ cargo build --target=arm-unknown-linux-gnueabi
```

Then you need to copy the executable onto Raspberry, configure it (see next header) and voilà!

Since during the development one usually does a lot of compiling and copying (at least I have), I've created a simple
script which simplifies this process (`make.sh` + `make-config.sh.example`) - you don't have to use it, unless you plan
on deploying this application frequently.

# Configuring

HomeStation needs a simple configuration to work - you have to create a file `config.hjson` in the application's
directory with following structure:

```javascript
{
  apis: {
    airly: {
      key: "...",
      sensor_id: "...",
    },
  },

  devices: {
    lcd: {
      i2c: {
        device: "/dev/i2c-1",
        address: "39",
      },
    },
  },
}
```

You can leave the [Airly](https://airly.eu) configuration empty, if you don't have a key (although I highly encourage
you to create a free account at their site and receive one, if you're from Poland).

After configuring, execute `./home-station` and watch the show.

# License

```
Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
Licensed under the MIT license.
```