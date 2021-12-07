import pandas as pd
import numpy as np

__all__ = ["basic_statistics"]


def basic_statistics(df, column):
    stats_df = df.groupby(column).agg(
        mean_opt=pd.NamedAgg(column="best_fx", aggfunc=np.mean),
        std_opt=pd.NamedAgg(column="best_fx", aggfunc=np.std),
        min_opt=pd.NamedAgg(column="best_fx", aggfunc="min"),
        max_opt=pd.NamedAgg(column="best_fx", aggfunc="max"),
        median_opt=pd.NamedAgg(column="best_fx", aggfunc=np.median),
    )
    pd.DataFrame(stats_df)
    return stats_df

