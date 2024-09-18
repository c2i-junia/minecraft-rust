# Variables
SOURCE_FILES = main.c
CFLAGS = -Wall -Wextra -std=c18
DESKTOP_FLAGS = -g -lGL -lm -lpthread -ldl -lrt -lX11

# Targets
.PHONY: debug release desktop clean

desktop:
	mkdir -p out/desktop
	gcc $(SOURCE_FILES) -Isrc/ libraylib.a $(CFLAGS) $(DESKTOP_FLAGS) -o prog
clean:
	rm -f prog
