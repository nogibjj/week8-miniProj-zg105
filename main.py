'''
Read from data.csv and calculate median, report CPU usage
'''
import time
import psutil
import pandas as pd
def get_median(data_frame):
    '''
    calculate the median
    '''
    return data_frame.median()

if __name__ == "__main__":
    start = time.time()
    CSV = "data.csv"
    df = pd.read_csv(CSV)
    print(get_median(df))
    end = time.time()
    time_spend = end - start
    cpu_percent = psutil.cpu_percent()
    memory_info = psutil.virtual_memory()
    print(f"Time Spent: {time_spend:.4f} seconds")
    print(f"CPU Usage: {cpu_percent}%")
    print(f"Memory Usage: {memory_info.percent}%")