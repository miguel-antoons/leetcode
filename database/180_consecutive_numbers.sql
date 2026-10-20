with triples as (
    select
        num,
        lead(num, 1) over (order by id) as next1,
        lead(num, 2) over (order by id) as next2
    from Logs
)
select distinct num as ConsecutiveNums
from triples
where num = next2 and num = next1


-- with triples as (
--     select
--         num,
--         id - row_number() over (partition by num order by id) as run_id
--     from Logs
-- )
-- select distinct num as ConsecutiveNums
-- from triples
-- group by num, run_id
-- having count(*) >= 3