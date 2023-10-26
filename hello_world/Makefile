

run:
	cargo run


debug:
	cargo run -- -S


## for macos-M1 there is no gdb, but we can run gdb in docker

docker:
	docker run -it --rm -v $PWD:/pwd -v $HOME:/home callaghanmt/xv6-tools:buildx-latest bash


GDBOPT := -iex 'add-auto-load-safe-path .'
GDBTARGET := -iex 'target remote host.docker.internal:1234'
GDBFILE := -iex 'file target/riscv32imac-unknown-none-elf/debug/a0-bare_metal'

gdb:
	gdb-multiarch ${GDBOPT} ${GDBTARGET} ${GDBFILE}

dock-gdb:
	docker run -it --rm -v ${PWD}:/pwd -v ${HOME}:/home callaghanmt/xv6-tools:buildx-latest \
		bash -c 'cd /pwd; make gdb'

