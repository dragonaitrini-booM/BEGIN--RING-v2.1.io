CLANG = clang
CFLAGS = -O2 -g -target bpf -c
TARGET = bpf_firewall.o
SOURCE = bpf_firewall.c

all: $(TARGET)

$(TARGET): $(SOURCE)
	$(CLANG) $(CFLAGS) $(SOURCE) -o $(TARGET)

clean:
	rm -f $(TARGET)
