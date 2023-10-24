from ai import run


def main():
    input_file = input('Enter the path of the file: ')
    output = run(input_file)
    print('~~SUMMARY~~')
    print(output)


if __name__ == '__main__':
    try:
        main()
    except Exception as e:
        print('Failed to run: ', e)
