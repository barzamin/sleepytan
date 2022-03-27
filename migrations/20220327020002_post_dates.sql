ALTER TABLE `post`
ADD COLUMN `create_ts` not null default (datetime('now'));
