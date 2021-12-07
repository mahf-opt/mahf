import pandas as pd
import os

__all__ = ["read_specific", "read_all", "read_experiment"]


# read specified csv file and convert it to dataframe
def read_specific(file_path):
    return pd.read_csv(file_path)


# read all csv files for one function, convert them to dataframes and return them as dict
def read_all(file_dir):
    directory = os.fsencode(file_dir)
    df_dict = {}
    for file in os.listdir(directory):
        filename = os.fsdecode(file)
        if filename.endswith(".csv"):
            df = pd.read_csv(file_dir + filename)
            df_dict[filename] = df
    return df_dict


# read all csv files of experiment, convert them to dataframes and return them as dict
def read_experiment(folder_dir):
    directory = os.fsencode(folder_dir)
    df_dict = {}
    for subdir in os.listdir(directory):
        subdir_name = os.fsdecode(subdir) + "/"
        for file in os.listdir(os.path.join(directory, subdir)):
            filename = os.fsdecode(file)
            if filename.endswith(".csv"):
                df = pd.read_csv(folder_dir + subdir_name + filename)
                df_dict[subdir_name + filename] = df
    return df_dict
