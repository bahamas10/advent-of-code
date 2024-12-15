#!/usr/bin/awk -f

BEGIN {
	accum = 0;
}

/$1 ~ \d/
{
	idx = NR - 1;
	num = $1;
	accum += idx * num;
}

END {
	printf("accum is %d\n", accum);
}
