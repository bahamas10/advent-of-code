#!/usr/bin/env bash

OPERATORS=('+' '*')

generate-operators() {
	local prefix=$1
	local num_opers=$2

	if ((num_opers == 0)); then
		# were done
		echo "$prefix"
		return
	fi

	local oper
	for oper in "${OPERATORS[@]}"; do
		local copy=$prefix
		copy+="$oper "
		generate-operators "$copy" "$((num_opers - 1))"
	done
}

total=0
while IFS=: read -r test_number numbers; do
	read -ra numbers <<< "$numbers"

	num_opers=$(( ${#numbers[@]} - 1))

	# generate all possible operators for this test
	while read -ra operators; do
		# we have 1 set of numbers and operators to test
		accum=${numbers[0]}
		for ((i = 0; i < ${#operators[@]}; i++)); do
			oper=${operators[i]}
			cur=${numbers[i+1]}

			case "$oper" in
				'*') ((accum *= cur));;
				'+') ((accum += cur));;
				*) echo "$oper bad"; exit 5;;
			esac
		done

		if ((accum == test_number)); then
			# we found a true result
			echo "made $test_number true with ${operators[*]}"
			((total += test_number))
			break
		fi
	done < <(generate-operators '' "$num_opers")
done

echo "final total = $total"
