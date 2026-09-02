CREATE OR REPLACE FUNCTION NthHighestSalary(N INT) RETURNS TABLE (Salary INT) AS $$
BEGIN
  if N < 1 then
    return query select null::int;
  else
    return query (
        select (
            select distinct e.salary
            from Employee e
            order by e.salary desc
            limit 1
            offset N - 1
        )
    );
  end if;
END;
$$ LANGUAGE plpgsql;