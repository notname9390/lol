#!/usr/bin/env python3

def greet(name):
    return f"Hello, {name}!"

def main():
    print("Hello from Python!")
    print(greet("World"))
    
    # Simple list comprehension
    numbers = [i * 2 for i in range(5)]
    print(f"Numbers: {numbers}")

if __name__ == "__main__":
    main() 