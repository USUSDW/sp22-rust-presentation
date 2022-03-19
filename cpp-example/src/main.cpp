#include <iostream>
#include <string>

template <typename T>
struct NamedPointer {
    std::string name;
    T* pointer;

    NamedPointer(std::string name, T* pointer):
        name(name), pointer(pointer) {}
};

void intPointerDemo();

template <typename T>
void printPointer(const NamedPointer<T> namedPointer);

void printSeparator();

int main() {
    intPointerDemo();
    printSeparator();

    return 0;
}

void intPointerDemo() {
    const auto foo = NamedPointer<int>(std::string("foo"), new int(22));
    printPointer(foo);

    *foo.pointer = 87;
    printPointer(foo);

    delete foo.pointer;
    printPointer(foo);
}

struct MyStruct {
    int firstMember;
    int secondMember;
};

template <typename T>
void printPointer(const NamedPointer<T> namedPointer) {
    std::cout << namedPointer.name << " (pointer): " << namedPointer.pointer  << std::endl;
    std::cout << namedPointer.name << " (value)  : " << *namedPointer.pointer << std::endl;
}

void printSeparator() {
    std::cout << "----------------" << std::endl;
}
