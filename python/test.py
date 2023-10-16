import datetime

def run(name):
    time = datetime.datetime.now()
    print(f'Hello {name}! The time is {time}')
    return str(time)

