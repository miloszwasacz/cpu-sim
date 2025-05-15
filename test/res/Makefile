CC = riscv64-unknown-elf-gcc
CXX = riscv64-unknown-elf-g++
CFLAGS = -mabi=lp64 -march=rv64im
CXXFLAGS = -mabi=lp64 -march=rv64im


SRC_DIR = ./src
LIB_DIR = $(SRC_DIR)/lib
INC_DIR = $(SRC_DIR)/include
OBJ_DIR = ./obj
BIN_DIR = ./bin

INC_FLAGS = -I $(INC_DIR)
LIB_CFLAGS = $(INC_FLAGS)
BIN_CFLAGS = $(INC_FLAGS) \
			 -flto -fuse-linker-plugin \
			 -ffunction-sections -Wl,--gc-sections \
			 -O1

# Ensure OBJ_DIR & BIN_DIR exist
$(shell mkdir -p $(OBJ_DIR))
$(shell mkdir -p $(BIN_DIR))

LIB_SRCS = $(wildcard $(LIB_DIR)/*.c)
LIB_OBJS = $(patsubst $(LIB_DIR)/%.c, $(OBJ_DIR)/%.o, $(LIB_SRCS))
BIN_SRCS = $(shell find $(SRC_DIR) -type f -name '*.c' -not -path "$(LIB_DIR)/*" -exec basename '{}' '.c' ';')

.PHONY: all clean

$(OBJ_DIR)/%.o: $(LIB_DIR)/%.c
	$(CC) $(CFLAGS) $(LIB_CFLAGS) -c $< -o $@

.PRECIOUS: $(OBJ_DIR)/%.o

all: $(BIN_SRCS)

%: $(SRC_DIR)/%.c $(LIB_OBJS)
	$(CC) $(CFLAGS) $(BIN_CFLAGS) $< $(LIB_OBJS) -o $(BIN_DIR)/$@
	@echo "Compiled \`$@\` successfully!"

clean:
	rm -rf $(OBJ_DIR) $(BIN_DIR)
