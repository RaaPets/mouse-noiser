help:
	@cat Makefile

winrun:
	@cargo run -r --target=x86_64-pc-windows-gnu

# # # # # # # #
pull:
	@git pull

savetogit: git.pushall
git.pushall: git.commitall
	@git push
git.commitall: git.addall
	@if [ -n "$(shell git status -s)" ] ; then git commit -m 'saving'; else echo '--- nothing to commit'; fi
git.addall:
	@git add .

# # # # # # # #
clean:
	@cargo clean
