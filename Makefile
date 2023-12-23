build:
	gcc src/*.c src/*.h src/vars/*.h src/led1/*.c src/led1/*.h src/led0/*.c src/led0/*.h src/leds/*.c src/leds/*.h -o build/recontrolled

clean: 
	rm build/recontrolled

run:
	$(build)
	@./build/recontrolled

install: 
	sudo rm -f /usr/bin/recontrolled
	$(clean)
	$(build)
	sudo cp build/recontrolled /usr/bin/recontrolled