import pandas as pd
import cbor2 as cb
import os

__all__ = ["read_log", "read_log_dir"]


def read_log(file_path: str) -> pd.DataFrame:
    """Read specified log file and convert it to dataframe"""
    with open(file_path, 'rb') as fp:
        obj = cb.load(fp)
    names = obj['names']
    entries = [{names[item['key']]: item['value'] for item in entry} for entry in obj['entries']]
    frame = pd.DataFrame.from_records(data=entries, columns=names)
    return frame


def read_log_dir(file_dir):
    """Read all log files for one function, convert them to dataframes and return them as dict"""
    directory = os.fsencode(file_dir)
    df_dict = {}
    for file in os.listdir(directory):
        filename = os.fsdecode(file)
        if filename.endswith(".log"):
            df = read_log(file_dir + filename)
            df_dict[filename] = df
    return df_dict
