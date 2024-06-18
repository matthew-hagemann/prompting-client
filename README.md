# Apparmor prompting local testing

> See [SD121][0] and [UD062][1] for specs covering the snapd and client
> implementations respectively.

![screenshot](./screenshot.png)

## Getting set up
```bash
$ sudo apt install virt-viewer
$ sudo snap install lxd
$ sudo snap install snapcraft --classic
```

## Local testing via lxd

So long as you have `lxd` installed you will be able to quickly spin up and
bootstrap a VM for local testing against the `latest/edge/prompting` channel
of snapd.

> If you have your own preferred workflow for local development and testing
> this is not a requirement, but it does help standardise how we test changes
> on both the client and snapd sides ;)

The first time you are setting up your test VM, running the following from
your host should be sufficient to get you a local testing environment. Please
note that you will need a local rust compiler to be able to build the client.
```bash
$ make prepare-vm
```

## Running the example CLI client

You should be able to run the following in order to demo the client
interaction with snapd for allowing and denying prompts:
```bash
$ make run-prompt-client

# In another terminal
$ make attach-vm-bash
# In the new bash shell
$ aa-prompting-test.read
```

## Running the integration tests

You may build and run the integration tests from your host like so:
```bash
$ make integration-tests
```

Note that this shares the same `aa-testing` VM as the other `make` directives.


## TODO
- [ ] Development of the actual client code


  [0]: https://docs.google.com/document/d/1tBnefdukP69EUJOlH8bgD2hrvZCYoE8-1ZlqRRYlOqc/edit
  [1]: https://docs.google.com/document/d/1zJVbo3rRc0yfNMTloE2vJGVldHLC0-PmxAyJoFn7mwE/edit
