import pandas as pd

__all__ = ["add_missing_values"]


def add_missing_values(df, column):
    """Add missing values to dataframe when data is logged on event (mostly evaluations)"""
    # gather all logged on event numbers
    loe_numbers = sorted(df[column].unique())
    loe_numbers = pd.DataFrame(loe_numbers, columns=[column])

    # split dataframe into several, one for each run
    splits = []
    start = 0
    for i in df.index:
        if df.at[i, column] == df[column].max():
            splits.append(df.iloc[start:i+1])
            start = i+1

    # expand evaluation numbers for each run and fill missing data with those of previous evaluation
    dfs = [j.merge(loe_numbers, on=column, how='right') for j in splits]
    df_filled = [k.fillna(method='ffill') for k in dfs]

    # merge now completed dataframes once more
    return pd.concat(df_filled)
