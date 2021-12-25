# Watchdog

*Watchdog* is a tool to allow you to keep track of your public IP address by
keeping the most up-to-date IP in a GitHub Gist. Knowing the public IP address
of one of your machines can be very helpful while you're on the road on another
machine. Watchdog lets you do that easily.

My ISP charges extra for static IPs, but I have found keeping the public address
accessible online to be a satisfactory alternative. I built this program for
myself as an upgrade to a dinky Python script I was using.

## Usage

Both the `update` and `fetch` commands require an ID of a GitHub Gist to be
passed as a positional argument. That may look something like:

```console
$ watchdog fetch 2be930b9e8f00d34619f748efb54def3
```

You should **create your own gist first**&mdash;simply use a blank or dummy file
to start out.

If you would prefer, you can use an environment variable instead:

```console
$ export IP_GIST_ID='2be930b9e8f00d34619f748efb54def3'
$ watchdog fetch -v IP_GIST_ID
```

This allows you to keep your Gist's ID private should you wish to put a call to
`watchdog` in a file that may be committed somewhere, such as `.bashrc` or
`.ssh/config` (as I intend to do) in a dot-files repo.

See `watchdog --help`, `watchdog update --help`, and `watchdog fetch --help` for
a complete set of options.


### Update

Queries https://ipinfo.io for the device's current public IP address and update
the given gist with it. This command should be run on a schedule using something
like `cron` or the Windows Task Scheduler.


#### Extra options

- `--force`, `-f`: Update the Gist even if it's most recent address is
  up-to-date.
- `--print`, `-p`: Print the new IP address to `stdout` after updating the Gist.
- `--use-ssh`, `-s`: Use the `git@gist.github.com` URL for cloning and pushing
  the Gist, rather than HTTPS. Because this program simply spawns `git`
  directly, you can select a specific SSH key by [setting an environnement
  variable](https://stackoverflow.com/a/29754018/10549827).
- `--dry-run`, `-d`: Fetch a new IP address from https://ipinfo.io and print it,
  without updating any Gist.


### Fetch

Run `watchdog fetch` and pass an ID of a GitHub Gist to query it for the most
up-to-date public IP address. This is intended to be used from another computer
to get the address of the main computer, such as when you're attempting to
establish SSH connection.


## Installation

You can also install `watchdog` with Cargo by using

```console
$ cargo install --git https://github.com/matthew-e-brown/watchdog
```