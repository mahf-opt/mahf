### A Pluto.jl notebook ###
# v0.14.7

using Markdown
using InteractiveUtils

# ╔═╡ 28ee4ea6-c66f-406a-b08c-84e334a18725
begin
	using CSV
	using DataFrames
	using Plots
end

# ╔═╡ e40a7b4e-2b15-4bef-b294-554a5d987068
data_dir = "../data/aco/"

# ╔═╡ fc7ea2d0-c159-46f2-8e7a-4f21b2d44286
as_iter = CSV.File(data_dir * "as/iterations.csv") |> DataFrame

# ╔═╡ 19dc956e-7512-4bd6-ae7d-680a5812dd53
as_eval = CSV.File(data_dir * "as/evaluations.csv") |> DataFrame

# ╔═╡ 2629ca1c-4961-4653-9f05-dd174a6ed3cd
mmas_iter = CSV.File(data_dir * "mmas/iterations.csv") |> DataFrame

# ╔═╡ 2f012fac-54d8-4189-9a29-2d412a29b1e5
mmas_eval = CSV.File(data_dir * "mmas/evaluations.csv") |> DataFrame

# ╔═╡ 28495cb2-c2ff-4838-b23d-b028902d458c
begin
	plot(as_iter[!, :iteration], as_iter[!, :min_pheromone], label="as_min")
	plot!(as_iter[!, :iteration], as_iter[!, :max_pheromone], label="as_max")
	plot!(as_iter[!, :iteration], as_iter[!, :avg_pheromone], label="as_avg")
end

# ╔═╡ 5d54ad18-774d-4367-a7dc-37d2deddb42d
begin
	plot(mmas_iter[!, :iteration], mmas_iter[!, :min_pheromone], label="mmas_min")
	plot!(mmas_iter[!, :iteration], mmas_iter[!, :max_pheromone], label="mmas_max")
	plot!(mmas_iter[!, :iteration], mmas_iter[!, :avg_pheromone], label="mmas_avg")
end

# ╔═╡ d56510d5-519f-4467-ab15-2f034a0fd126
begin
	plot(as_eval[!, :evaluation], as_eval[!, :best_fx], label="as")
	plot!(mmas_eval[!, :evaluation], mmas_eval[!, :best_fx], label="mmas")
end

# ╔═╡ Cell order:
# ╠═28ee4ea6-c66f-406a-b08c-84e334a18725
# ╠═e40a7b4e-2b15-4bef-b294-554a5d987068
# ╠═fc7ea2d0-c159-46f2-8e7a-4f21b2d44286
# ╠═19dc956e-7512-4bd6-ae7d-680a5812dd53
# ╠═2629ca1c-4961-4653-9f05-dd174a6ed3cd
# ╠═2f012fac-54d8-4189-9a29-2d412a29b1e5
# ╠═28495cb2-c2ff-4838-b23d-b028902d458c
# ╠═5d54ad18-774d-4367-a7dc-37d2deddb42d
# ╠═d56510d5-519f-4467-ab15-2f034a0fd126
