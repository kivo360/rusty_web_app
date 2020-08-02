SELECT
    *
FROM (
    SELECT
        ROW_NUMBER() OVER (PARTITION BY event_type ORDER BY created_at) AS r,
        t.*
    FROM
        events t
    WHERE t.streamer_name=$1
) x
WHERE
    x.r <= $2;