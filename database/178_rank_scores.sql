-- with ordered_scores as (
--     select distinct score
--     from Scores
--     order by score desc
-- ), scores_rank as (
--     select score, row_number() over () as rank
--     from ordered_scores
-- )
-- select s.score, r.rank
-- from scores_rank r
-- join Scores s on r.score = s.score
-- order by r.rank asc
-- 
select score, dense_rank() over (order by score desc) as rank
from Scores
order by score desc