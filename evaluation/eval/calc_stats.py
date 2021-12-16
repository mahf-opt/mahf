import pandas as pd
import numpy as np

__all__ = ["basic_statistics", "diversity_statistics"]


# calculate basic statistics for best found solution depending on column (usually either iterations or evaluations)
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


# calculate basic statistics for population diversity depending on column (usually either iterations or evaluations)
def diversity_statistics(df, column):
    stats_df = df.groupby(column).agg(
        mean_div=pd.NamedAgg(column="diversity", aggfunc=np.mean),
        std_div=pd.NamedAgg(column="diversity", aggfunc=np.std),
        min_div=pd.NamedAgg(column="diversity", aggfunc="min"),
        max_div=pd.NamedAgg(column="diversity", aggfunc="max"),
        median_div=pd.NamedAgg(column="diversity", aggfunc=np.median),
    )
    pd.DataFrame(stats_df)
    return stats_df
