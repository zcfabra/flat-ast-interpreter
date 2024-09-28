import datetime as dt

from dis import dis
from time import sleep


def do_thing_in_time_range(
    start: dt.datetime = dt.datetime.now() - dt.timedelta(days=15),
    end: dt.datetime = dt.datetime.now()
):
    print("Default Arg: ", end)
    print("Now: ", dt.datetime.now())


def thing():
    while True:
        do_thing_in_time_range()
        sleep(5)

dis(thing)
