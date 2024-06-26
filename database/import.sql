-- This script will create the necessary tables and import all data into an
-- existing PostgreSQL database.
--
-- Usage:
--   Run a 'CREATE DATABASE $database' somewhere.
--   psql -U $user $database -f import.sql
--
-- The imported database does not include any indices, other than primary keys.
-- You may want to create some indices by hand to speed up complex queries.

-- Uncomment to import the schema and data into a separate namespace:
--CREATE SCHEMA vndb;
--SET search_path TO vndb;

-- 'vndbid' is a custom base type used in the VNDB codebase, but it's safe to treat
-- it as just text. If you want to use the proper type, load sql/vndbid.sql from
-- the VNDB source code into your database and comment out the following line.
-- (or ignore the error message about 'vndbid' already existing)
CREATE DOMAIN vndbid AS text;


CREATE DOMAIN animation AS smallint CHECK(value IS NULL OR value IN(0,1) OR ((value & (4+8+16+32)) > 0 AND (value & (256+512)) <> (256+512)));
CREATE TYPE anime_type        AS ENUM ('tv', 'ova', 'mov', 'oth', 'web', 'spe', 'mv');
CREATE TYPE blood_type        AS ENUM ('unknown', 'a', 'b', 'ab', 'o');
CREATE TYPE char_role         AS ENUM ('main', 'primary', 'side', 'appears');
CREATE TYPE credit_type       AS ENUM ('scenario', 'chardesign', 'art', 'music', 'songs', 'director', 'translator', 'editor', 'qa', 'staff');
CREATE TYPE cup_size          AS ENUM ('', 'AAA', 'AA', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z');
CREATE TYPE gender            AS ENUM ('unknown', 'm', 'f', 'b');
CREATE TYPE language          AS ENUM ('ar', 'be', 'bg', 'ca', 'cs', 'ck', 'da', 'de', 'el', 'en', 'eo', 'es', 'eu', 'fa', 'fi', 'fr', 'ga', 'gd', 'he', 'hi', 'hr', 'hu', 'id', 'it', 'iu', 'ja', 'ko', 'mk', 'ms', 'la', 'lt', 'lv', 'nl', 'no', 'pl', 'pt-pt', 'pt-br', 'ro', 'ru', 'sk', 'sl', 'sr', 'sv', 'ta', 'th', 'tr', 'uk', 'ur', 'vi', 'zh', 'zh-Hans', 'zh-Hant');
CREATE TYPE medium            AS ENUM ('cd', 'dvd', 'gdr', 'blr', 'flp', 'cas', 'mrt', 'mem', 'umd', 'nod', 'in', 'dc', 'otc');
CREATE TYPE platform          AS ENUM ('win', 'dos', 'lin', 'mac', 'ios', 'and', 'dvd', 'bdp', 'fm7', 'fm8', 'fmt', 'gba', 'gbc', 'msx', 'nds', 'nes', 'p88', 'p98', 'pce', 'pcf', 'psp', 'ps1', 'ps2', 'ps3', 'ps4', 'ps5', 'psv', 'drc', 'smd', 'scd', 'sat', 'sfc', 'swi', 'wii', 'wiu', 'n3d', 'vnd', 'x1s', 'x68', 'xb1', 'xb3', 'xbo', 'xxs', 'web', 'tdo', 'mob', 'oth');
CREATE TYPE producer_relation AS ENUM ('old', 'new', 'sub', 'par', 'imp', 'ipa', 'spa', 'ori');
CREATE TYPE producer_type     AS ENUM ('co', 'in', 'ng');
CREATE TYPE release_image_type AS ENUM ('pkgfront', 'pkgback', 'pkgcontent', 'pkgside', 'pkgmed', 'dig');
CREATE TYPE release_type      AS ENUM ('complete', 'partial', 'trial');
CREATE TYPE tag_category      AS ENUM('cont', 'ero', 'tech');
CREATE TYPE vn_relation       AS ENUM ('seq', 'preq', 'set', 'alt', 'char', 'side', 'par', 'ser', 'fan', 'orig');

CREATE TABLE anime (
  id integer NOT NULL,
  ann_id integer,
  type anime_type,
  year smallint,
  nfo_id varchar(200),
  title_romaji varchar(250),
  title_kanji varchar(250),
  PRIMARY KEY(id)
);

CREATE TABLE chars (
  id vndbid NOT NULL,
  image vndbid,
  gender gender NOT NULL,
  spoil_gender gender,
  bloodt blood_type NOT NULL,
  cup_size cup_size NOT NULL,
  main vndbid,
  s_bust smallint NOT NULL,
  s_waist smallint NOT NULL,
  s_hip smallint NOT NULL,
  b_month smallint NOT NULL,
  b_day smallint NOT NULL,
  height smallint NOT NULL,
  weight smallint,
  main_spoil smallint NOT NULL,
  age smallint,
  name varchar(250) NOT NULL,
  latin varchar(250),
  alias varchar(500) NOT NULL,
  description text NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE chars_traits (
  id vndbid NOT NULL,
  tid vndbid NOT NULL,
  spoil smallint NOT NULL,
  lie boolean NOT NULL,
  PRIMARY KEY(id, tid)
);

CREATE TABLE chars_vns (
  id vndbid NOT NULL,
  vid vndbid NOT NULL,
  rid vndbid NULL,
  role char_role NOT NULL,
  spoil smallint NOT NULL
);

CREATE TABLE docs (
  id vndbid NOT NULL,
  title varchar(200) NOT NULL,
  content text NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE image_votes (
  id vndbid NOT NULL,
  uid vndbid,
  date timestamptz NOT NULL,
  sexual smallint NOT NULL CHECK(sexual >= 0 AND sexual <= 2),
  violence smallint NOT NULL CHECK(violence >= 0 AND violence <= 2),
  ignore boolean NOT NULL
);

CREATE TABLE images (
  id vndbid NOT NULL,
  width smallint NOT NULL,
  height smallint NOT NULL,
  c_votecount smallint NOT NULL,
  c_sexual_avg smallint NOT NULL,
  c_sexual_stddev smallint NOT NULL,
  c_violence_avg smallint NOT NULL,
  c_violence_stddev smallint NOT NULL,
  c_weight smallint NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE producers (
  id vndbid NOT NULL,
  type producer_type NOT NULL,
  lang language NOT NULL,
  l_wikidata integer,
  name varchar(200) NOT NULL,
  latin varchar(200),
  alias varchar(500) NOT NULL,
  website varchar(1024) NOT NULL,
  description text NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE producers_relations (
  id vndbid NOT NULL,
  pid vndbid NOT NULL,
  relation producer_relation NOT NULL,
  PRIMARY KEY(id, pid)
);

CREATE TABLE quotes (
  id integer,
  vid vndbid NOT NULL,
  cid vndbid,
  score smallint NOT NULL,
  quote text NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE releases (
  id vndbid NOT NULL,
  olang language NOT NULL,
  gtin bigint NOT NULL,
  l_toranoana bigint NOT NULL,
  l_appstore bigint NOT NULL,
  l_nintendo_jp text NOT NULL,
  l_nintendo_hk bigint NOT NULL,
  released integer NOT NULL,
  l_steam integer NOT NULL,
  l_digiket integer NOT NULL,
  l_melon integer NOT NULL,
  l_mg integer NOT NULL,
  l_getchu integer NOT NULL,
  l_getchudl integer NOT NULL,
  l_egs integer NOT NULL,
  l_erotrail integer NOT NULL,
  l_melonjp integer NOT NULL,
  l_gamejolt integer NOT NULL,
  l_animateg integer NOT NULL,
  l_freem integer NOT NULL,
  l_novelgam integer NOT NULL,
  voiced smallint NOT NULL,
  reso_x smallint NOT NULL,
  reso_y smallint NOT NULL,
  minage smallint,
  ani_story smallint NOT NULL,
  ani_ero smallint NOT NULL,
  ani_story_sp animation,
  ani_story_cg animation,
  ani_cutscene animation,
  ani_ero_sp animation,
  ani_ero_cg animation,
  ani_bg boolean,
  ani_face boolean,
  has_ero boolean NOT NULL,
  patch boolean NOT NULL,
  freeware boolean NOT NULL,
  uncensored boolean,
  official boolean NOT NULL,
  website varchar(1024) NOT NULL,
  catalog varchar(50) NOT NULL,
  engine varchar(50) NOT NULL,
  notes text NOT NULL,
  l_dlsite text NOT NULL,
  l_gog text NOT NULL,
  l_denpa text NOT NULL,
  l_jlist text NOT NULL,
  l_jastusa text NOT NULL,
  l_itch text NOT NULL,
  l_nutaku text NOT NULL,
  l_googplay text NOT NULL,
  l_fakku text NOT NULL,
  l_freegame text NOT NULL,
  l_playstation_jp text NOT NULL,
  l_playstation_na text NOT NULL,
  l_playstation_eu text NOT NULL,
  l_playstation_hk text NOT NULL,
  l_nintendo text NOT NULL,
  l_gyutto integer[] NOT NULL,
  l_dmm text[] NOT NULL,
  l_booth integer NOT NULL,
  l_patreonp integer NOT NULL,
  l_patreon text NOT NULL,
  l_substar text NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE releases_images (
  id vndbid NOT NULL,
  img vndbid NOT NULL,
  itype release_image_type NOT NULL,
  vid vndbid,
  lang language[],
  PRIMARY KEY(id, img)
);

CREATE TABLE releases_media (
  id vndbid NOT NULL,
  medium medium NOT NULL,
  qty smallint NOT NULL,
  PRIMARY KEY(id, medium, qty)
);

CREATE TABLE releases_platforms (
  id vndbid NOT NULL,
  platform platform NOT NULL,
  PRIMARY KEY(id, platform)
);

CREATE TABLE releases_producers (
  id vndbid NOT NULL,
  pid vndbid NOT NULL,
  developer boolean NOT NULL,
  publisher boolean NOT NULL,
  PRIMARY KEY(id, pid)
);

CREATE TABLE releases_supersedes (
  id vndbid NOT NULL,
  rid vndbid NOT NULL,
  PRIMARY KEY(id, rid)
);

CREATE TABLE releases_titles (
  id vndbid NOT NULL,
  lang language NOT NULL,
  mtl boolean NOT NULL,
  title text,
  latin text,
  PRIMARY KEY(id, lang)
);

CREATE TABLE releases_vn (
  id vndbid NOT NULL,
  vid vndbid NOT NULL,
  rtype release_type NOT NULL,
  PRIMARY KEY(id, vid)
);

CREATE TABLE rlists (
  uid vndbid NOT NULL,
  rid vndbid NOT NULL,
  added timestamptz NOT NULL,
  status smallint NOT NULL,
  PRIMARY KEY(uid, rid)
);

CREATE TABLE staff (
  id vndbid NOT NULL,
  gender gender NOT NULL,
  lang language NOT NULL,
  main integer NOT NULL,
  l_anidb integer,
  l_wikidata integer,
  l_pixiv integer NOT NULL,
  description text NOT NULL,
  l_site varchar(250) NOT NULL,
  l_twitter varchar(16) NOT NULL,
  l_vgmdb integer NOT NULL,
  l_discogs integer NOT NULL,
  l_mobygames integer NOT NULL,
  l_bgmtv integer NOT NULL,
  l_imdb integer NOT NULL,
  l_vndb vndbid,
  l_mbrainz uuid,
  l_scloud text NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE staff_alias (
  id vndbid NOT NULL,
  aid integer,
  name varchar(200) NOT NULL,
  latin varchar(200),
  PRIMARY KEY(aid)
);

CREATE TABLE tags (
  id vndbid NOT NULL,
  cat tag_category NOT NULL,
  defaultspoil smallint NOT NULL,
  searchable boolean NOT NULL,
  applicable boolean NOT NULL,
  name varchar(250) NOT NULL,
  alias varchar(500) NOT NULL,
  description text NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE tags_parents (
  id vndbid NOT NULL,
  parent vndbid NOT NULL,
  main boolean NOT NULL,
  PRIMARY KEY(id, parent)
);

CREATE TABLE tags_vn (
  date timestamptz NOT NULL,
  tag vndbid NOT NULL,
  vid vndbid NOT NULL,
  uid vndbid,
  vote smallint NOT NULL,
  spoiler smallint CHECK(spoiler >= 0 AND spoiler <= 2),
  ignore boolean NOT NULL,
  lie boolean,
  notes text NOT NULL
);

CREATE TABLE traits (
  id vndbid NOT NULL,
  gid vndbid,
  gorder smallint NOT NULL,
  defaultspoil smallint NOT NULL,
  sexual boolean NOT NULL,
  searchable boolean NOT NULL,
  applicable boolean NOT NULL,
  name varchar(250) NOT NULL,
  alias varchar(500) NOT NULL,
  description text NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE traits_parents (
  id vndbid NOT NULL,
  parent vndbid NOT NULL,
  main boolean NOT NULL,
  PRIMARY KEY(id, parent)
);

CREATE TABLE ulist_labels (
  uid vndbid NOT NULL,
  id smallint NOT NULL,
  label text NOT NULL,
  PRIMARY KEY(uid, id)
);

CREATE TABLE ulist_vns (
  uid vndbid NOT NULL,
  vid vndbid NOT NULL,
  added timestamptz NOT NULL,
  lastmod timestamptz NOT NULL,
  vote_date timestamptz,
  started date,
  finished date,
  vote smallint CHECK(vote IS NULL OR vote BETWEEN 10 AND 100),
  notes text NOT NULL,
  labels smallint[] NOT NULL,
  PRIMARY KEY(uid, vid)
);

CREATE TABLE users (
  id vndbid NOT NULL,
  ign_votes boolean NOT NULL,
  perm_imgvote boolean NOT NULL,
  perm_tag boolean NOT NULL,
  perm_lengthvote boolean NOT NULL,
  username varchar(20),
  PRIMARY KEY(id)
);

CREATE TABLE vn (
  id vndbid NOT NULL,
  olang language NOT NULL,
  image vndbid,
  l_wikidata integer,
  c_votecount integer NOT NULL,
  c_rating smallint,
  c_average smallint,
  length smallint NOT NULL,
  devstatus smallint NOT NULL,
  alias varchar(500) NOT NULL,
  l_renai varchar(100) NOT NULL,
  description text NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE vn_anime (
  id vndbid NOT NULL,
  aid integer NOT NULL,
  PRIMARY KEY(id, aid)
);

CREATE TABLE vn_editions (
  id vndbid NOT NULL,
  lang language,
  eid smallint NOT NULL,
  official boolean NOT NULL,
  name text NOT NULL,
  PRIMARY KEY(id, eid)
);

CREATE TABLE vn_length_votes (
  vid vndbid NOT NULL,
  date timestamptz NOT NULL,
  length smallint NOT NULL,
  speed smallint,
  uid vndbid,
  rid vndbid[] NOT NULL,
  notes text NOT NULL
);

CREATE TABLE vn_relations (
  id vndbid NOT NULL,
  vid vndbid NOT NULL,
  relation vn_relation NOT NULL,
  official boolean NOT NULL,
  PRIMARY KEY(id, vid)
);

CREATE TABLE vn_screenshots (
  id vndbid NOT NULL,
  scr vndbid NOT NULL,
  rid vndbid,
  PRIMARY KEY(id, scr)
);

CREATE TABLE vn_seiyuu (
  id vndbid NOT NULL,
  aid integer NOT NULL,
  cid vndbid NOT NULL,
  note varchar(250) NOT NULL,
  PRIMARY KEY(id, aid, cid)
);

CREATE TABLE vn_staff (
  id vndbid NOT NULL,
  aid integer NOT NULL,
  role credit_type NOT NULL,
  eid smallint,
  note varchar(250) NOT NULL
);

CREATE TABLE vn_titles (
  id vndbid NOT NULL,
  lang language NOT NULL,
  official boolean NOT NULL,
  title text NOT NULL,
  latin text,
  PRIMARY KEY(id, lang)
);

CREATE TABLE wikidata (
  id integer NOT NULL,
  enwiki text,
  jawiki text,
  website text[],
  vndb text[],
  mobygames text[],
  mobygames_company text[],
  gamefaqs_game integer[],
  gamefaqs_company integer[],
  anidb_anime integer[],
  anidb_person integer[],
  ann_anime integer[],
  ann_manga integer[],
  musicbrainz_artist uuid[],
  twitter text[],
  vgmdb_product integer[],
  vgmdb_artist integer[],
  discogs_artist integer[],
  acdb_char integer[],
  acdb_source integer[],
  indiedb_game text[],
  howlongtobeat integer[],
  crunchyroll text[],
  igdb_game text[],
  giantbomb text[],
  pcgamingwiki text[],
  steam integer[],
  gog text[],
  pixiv_user integer[],
  doujinshi_author integer[],
  soundcloud text[],
  humblestore text[],
  itchio text[],
  playstation_jp text[],
  playstation_na text[],
  playstation_eu text[],
  lutris text[],
  wine integer[],
  PRIMARY KEY(id)
);


-- You can comment out tables you don't need, to speed up the import and save some disk space.
\copy anime from 'db/anime'
\copy chars from 'db/chars'
\copy chars_traits from 'db/chars_traits'
\copy chars_vns from 'db/chars_vns'
\copy docs from 'db/docs'
\copy image_votes from 'db/image_votes'
\copy images from 'db/images'
\copy producers from 'db/producers'
\copy producers_relations from 'db/producers_relations'
\copy quotes from 'db/quotes'
\copy releases from 'db/releases'
\copy releases_images from 'db/releases_images'
\copy releases_media from 'db/releases_media'
\copy releases_platforms from 'db/releases_platforms'
\copy releases_producers from 'db/releases_producers'
\copy releases_supersedes from 'db/releases_supersedes'
\copy releases_titles from 'db/releases_titles'
\copy releases_vn from 'db/releases_vn'
\copy rlists from 'db/rlists'
\copy staff from 'db/staff'
\copy staff_alias from 'db/staff_alias'
\copy tags from 'db/tags'
\copy tags_parents from 'db/tags_parents'
\copy tags_vn from 'db/tags_vn'
\copy traits from 'db/traits'
\copy traits_parents from 'db/traits_parents'
\copy ulist_labels from 'db/ulist_labels'
\copy ulist_vns from 'db/ulist_vns'
\copy users from 'db/users'
\copy vn from 'db/vn'
\copy vn_anime from 'db/vn_anime'
\copy vn_editions from 'db/vn_editions'
\copy vn_length_votes from 'db/vn_length_votes'
\copy vn_relations from 'db/vn_relations'
\copy vn_screenshots from 'db/vn_screenshots'
\copy vn_seiyuu from 'db/vn_seiyuu'
\copy vn_staff from 'db/vn_staff'
\copy vn_titles from 'db/vn_titles'
\copy wikidata from 'db/wikidata'


-- These are included to verify the internal consistency of the dump, you can safely comment out this part.
ALTER TABLE chars                    ADD CONSTRAINT chars_main_fkey                    FOREIGN KEY (main)      REFERENCES chars         (id);
ALTER TABLE chars                    ADD CONSTRAINT chars_image_fkey                   FOREIGN KEY (image)     REFERENCES images        (id);
ALTER TABLE chars_traits             ADD CONSTRAINT chars_traits_id_fkey               FOREIGN KEY (id)        REFERENCES chars         (id);
ALTER TABLE chars_traits             ADD CONSTRAINT chars_traits_tid_fkey              FOREIGN KEY (tid)       REFERENCES traits        (id);
ALTER TABLE chars_vns                ADD CONSTRAINT chars_vns_id_fkey                  FOREIGN KEY (id)        REFERENCES chars         (id);
ALTER TABLE chars_vns                ADD CONSTRAINT chars_vns_vid_fkey                 FOREIGN KEY (vid)       REFERENCES vn            (id);
ALTER TABLE chars_vns                ADD CONSTRAINT chars_vns_rid_fkey                 FOREIGN KEY (rid)       REFERENCES releases      (id);
ALTER TABLE image_votes              ADD CONSTRAINT image_votes_id_fkey                FOREIGN KEY (id)        REFERENCES images        (id) ON DELETE CASCADE;
ALTER TABLE image_votes              ADD CONSTRAINT image_votes_uid_fkey               FOREIGN KEY (uid)       REFERENCES users         (id) ON DELETE SET DEFAULT;
ALTER TABLE producers                ADD CONSTRAINT producers_l_wikidata_fkey          FOREIGN KEY (l_wikidata)REFERENCES wikidata      (id);
ALTER TABLE producers_relations      ADD CONSTRAINT producers_relations_pid_fkey       FOREIGN KEY (pid)       REFERENCES producers     (id);
ALTER TABLE quotes                   ADD CONSTRAINT quotes_vid_fkey                    FOREIGN KEY (vid)       REFERENCES vn            (id);
ALTER TABLE quotes                   ADD CONSTRAINT quotes_cid_fkey                    FOREIGN KEY (cid)       REFERENCES chars         (id);
ALTER TABLE releases                 ADD CONSTRAINT releases_olang_fkey                FOREIGN KEY (id,olang)  REFERENCES releases_titles(id,lang) DEFERRABLE INITIALLY DEFERRED;
ALTER TABLE releases_images          ADD CONSTRAINT releases_images_id_fkey            FOREIGN KEY (id)        REFERENCES releases      (id);
ALTER TABLE releases_images          ADD CONSTRAINT releases_images_img_fkey           FOREIGN KEY (img)       REFERENCES images        (id);
ALTER TABLE releases_images          ADD CONSTRAINT releases_images_vid_fkey           FOREIGN KEY (id,vid)    REFERENCES releases_vn   (id,vid) DEFERRABLE INITIALLY DEFERRED;
ALTER TABLE releases_titles          ADD CONSTRAINT releases_titles_id_fkey            FOREIGN KEY (id)        REFERENCES releases      (id);
ALTER TABLE releases_media           ADD CONSTRAINT releases_media_id_fkey             FOREIGN KEY (id)        REFERENCES releases      (id);
ALTER TABLE releases_platforms       ADD CONSTRAINT releases_platforms_id_fkey         FOREIGN KEY (id)        REFERENCES releases      (id);
ALTER TABLE releases_producers       ADD CONSTRAINT releases_producers_id_fkey         FOREIGN KEY (id)        REFERENCES releases      (id);
ALTER TABLE releases_producers       ADD CONSTRAINT releases_producers_pid_fkey        FOREIGN KEY (pid)       REFERENCES producers     (id);
ALTER TABLE releases_supersedes      ADD CONSTRAINT releases_supersedes_id_fkey        FOREIGN KEY (id)        REFERENCES releases      (id);
ALTER TABLE releases_supersedes      ADD CONSTRAINT releases_supersedes_rid_fkey       FOREIGN KEY (rid)       REFERENCES releases      (id);
ALTER TABLE releases_vn              ADD CONSTRAINT releases_vn_id_fkey                FOREIGN KEY (id)        REFERENCES releases      (id);
ALTER TABLE releases_vn              ADD CONSTRAINT releases_vn_vid_fkey               FOREIGN KEY (vid)       REFERENCES vn            (id);
ALTER TABLE rlists                   ADD CONSTRAINT rlists_uid_fkey                    FOREIGN KEY (uid)       REFERENCES users         (id) ON DELETE CASCADE;
ALTER TABLE rlists                   ADD CONSTRAINT rlists_rid_fkey                    FOREIGN KEY (rid)       REFERENCES releases      (id);
ALTER TABLE staff                    ADD CONSTRAINT staff_main_fkey                    FOREIGN KEY (main)      REFERENCES staff_alias   (aid) DEFERRABLE INITIALLY DEFERRED;
ALTER TABLE staff                    ADD CONSTRAINT staff_l_wikidata_fkey              FOREIGN KEY (l_wikidata)REFERENCES wikidata      (id);
ALTER TABLE staff_alias              ADD CONSTRAINT staff_alias_id_fkey                FOREIGN KEY (id)        REFERENCES staff         (id);
ALTER TABLE tags_parents             ADD CONSTRAINT tags_parents_id_fkey               FOREIGN KEY (id)        REFERENCES tags          (id);
ALTER TABLE tags_parents             ADD CONSTRAINT tags_parents_parent_fkey           FOREIGN KEY (parent)    REFERENCES tags          (id);
ALTER TABLE tags_vn                  ADD CONSTRAINT tags_vn_tag_fkey                   FOREIGN KEY (tag)       REFERENCES tags          (id);
ALTER TABLE tags_vn                  ADD CONSTRAINT tags_vn_vid_fkey                   FOREIGN KEY (vid)       REFERENCES vn            (id);
ALTER TABLE tags_vn                  ADD CONSTRAINT tags_vn_uid_fkey                   FOREIGN KEY (uid)       REFERENCES users         (id) ON DELETE SET DEFAULT;
ALTER TABLE traits                   ADD CONSTRAINT traits_gid_fkey                    FOREIGN KEY (gid)       REFERENCES traits        (id);
ALTER TABLE traits_parents           ADD CONSTRAINT traits_parents_id_fkey             FOREIGN KEY (id)        REFERENCES traits        (id);
ALTER TABLE traits_parents           ADD CONSTRAINT traits_parents_parent_fkey         FOREIGN KEY (parent)    REFERENCES traits        (id);
ALTER TABLE ulist_labels             ADD CONSTRAINT ulist_labels_uid_fkey              FOREIGN KEY (uid)       REFERENCES users         (id) ON DELETE CASCADE;
ALTER TABLE ulist_vns                ADD CONSTRAINT ulist_vns_uid_fkey                 FOREIGN KEY (uid)       REFERENCES users         (id) ON DELETE CASCADE;
ALTER TABLE ulist_vns                ADD CONSTRAINT ulist_vns_vid_fkey                 FOREIGN KEY (vid)       REFERENCES vn            (id);
ALTER TABLE vn                       ADD CONSTRAINT vn_image_fkey                      FOREIGN KEY (image)     REFERENCES images        (id);
ALTER TABLE vn                       ADD CONSTRAINT vn_l_wikidata_fkey                 FOREIGN KEY (l_wikidata)REFERENCES wikidata      (id);
ALTER TABLE vn                       ADD CONSTRAINT vn_olang_fkey                      FOREIGN KEY (id,olang)  REFERENCES vn_titles     (id,lang)   DEFERRABLE INITIALLY DEFERRED;
ALTER TABLE vn_anime                 ADD CONSTRAINT vn_anime_id_fkey                   FOREIGN KEY (id)        REFERENCES vn            (id);
ALTER TABLE vn_anime                 ADD CONSTRAINT vn_anime_aid_fkey                  FOREIGN KEY (aid)       REFERENCES anime         (id);
ALTER TABLE vn_relations             ADD CONSTRAINT vn_relations_id_fkey               FOREIGN KEY (id)        REFERENCES vn            (id);
ALTER TABLE vn_relations             ADD CONSTRAINT vn_relations_vid_fkey              FOREIGN KEY (vid)       REFERENCES vn            (id);
ALTER TABLE vn_screenshots           ADD CONSTRAINT vn_screenshots_id_fkey             FOREIGN KEY (id)        REFERENCES vn            (id);
ALTER TABLE vn_screenshots           ADD CONSTRAINT vn_screenshots_scr_fkey            FOREIGN KEY (scr)       REFERENCES images        (id);
ALTER TABLE vn_screenshots           ADD CONSTRAINT vn_screenshots_rid_fkey            FOREIGN KEY (rid)       REFERENCES releases      (id);
ALTER TABLE vn_seiyuu                ADD CONSTRAINT vn_seiyuu_id_fkey                  FOREIGN KEY (id)        REFERENCES vn            (id);
ALTER TABLE vn_seiyuu                ADD CONSTRAINT vn_seiyuu_aid_fkey                 FOREIGN KEY (aid)       REFERENCES staff_alias   (aid) DEFERRABLE INITIALLY DEFERRED;
ALTER TABLE vn_seiyuu                ADD CONSTRAINT vn_seiyuu_cid_fkey                 FOREIGN KEY (cid)       REFERENCES chars         (id);
ALTER TABLE vn_staff                 ADD CONSTRAINT vn_staff_id_eid_fkey               FOREIGN KEY (id,eid)    REFERENCES vn_editions   (id,eid) DEFERRABLE INITIALLY DEFERRED;
ALTER TABLE vn_staff                 ADD CONSTRAINT vn_staff_aid_fkey                  FOREIGN KEY (aid)       REFERENCES staff_alias   (aid) DEFERRABLE INITIALLY DEFERRED;
ALTER TABLE vn_titles                ADD CONSTRAINT vn_titles_id_fkey                  FOREIGN KEY (id)        REFERENCES vn            (id);
ALTER TABLE vn_length_votes          ADD CONSTRAINT vn_length_votes_vid_fkey           FOREIGN KEY (vid)       REFERENCES vn            (id);
ALTER TABLE vn_length_votes          ADD CONSTRAINT vn_length_votes_uid_fkey           FOREIGN KEY (uid)       REFERENCES users         (id) ON DELETE SET DEFAULT;


-- Sparse documentation, but it's something!
COMMENT ON TABLE anime IS 'Anime information fetched from AniDB, only used for linking with visual novels.';
COMMENT ON COLUMN anime.id IS 'AniDB identifier';
COMMENT ON COLUMN anime.ann_id IS 'Anime News Network identifier';
COMMENT ON COLUMN anime.nfo_id IS 'AnimeNFO identifier (unused, site is long dead)';
COMMENT ON COLUMN chars.gender IS 'Character''s sex, not gender';
COMMENT ON COLUMN chars.spoil_gender IS 'Character''s actual sex, in case it''s a spoiler';
COMMENT ON COLUMN chars.bloodt IS 'Blood type';
COMMENT ON COLUMN chars.main IS 'When this character is an instance of another character';
COMMENT ON COLUMN chars.s_bust IS 'cm';
COMMENT ON COLUMN chars.s_waist IS 'cm';
COMMENT ON COLUMN chars.s_hip IS 'cm';
COMMENT ON COLUMN chars.b_month IS 'Birthday month, 1-12';
COMMENT ON COLUMN chars.b_day IS 'Birthday day, 1-32';
COMMENT ON COLUMN chars.height IS 'cm';
COMMENT ON COLUMN chars.weight IS 'kg';
COMMENT ON COLUMN chars.age IS 'years';
COMMENT ON COLUMN docs.content IS 'In MultiMarkdown format';
COMMENT ON COLUMN image_votes.sexual IS '0 = safe, 1 = suggestive, 2 = explicit';
COMMENT ON COLUMN image_votes.violence IS '0 = tame, 1 = violent, 2 = brutal';
COMMENT ON COLUMN image_votes.ignore IS 'Set when overruled by a moderator';
COMMENT ON COLUMN images.width IS 'px';
COMMENT ON COLUMN images.height IS 'px';
COMMENT ON COLUMN images.c_sexual_avg IS '0 - 200, so average vote * 100';
COMMENT ON COLUMN images.c_violence_avg IS '0 - 200';
COMMENT ON COLUMN images.c_weight IS 'Random selection weight for the image flagging UI';
COMMENT ON COLUMN producers.l_wikidata IS 'Wikidata, https://www.wikidata.org/wiki/Q%d ';
COMMENT ON COLUMN producers.website IS 'Official website, %s ';
COMMENT ON COLUMN releases.olang IS 'Refers to the main title to use for display purposes, not necessarily the original language.';
COMMENT ON COLUMN releases.gtin IS 'JAN/UPC/EAN/ISBN';
COMMENT ON COLUMN releases.l_toranoana IS 'Toranoana, https://ec.toranoana.shop/tora/ec/item/%012d/ ';
COMMENT ON COLUMN releases.l_appstore IS 'App Store, https://apps.apple.com/app/id%d ';
COMMENT ON COLUMN releases.l_nintendo_jp IS 'Nintendo (JP), https://store-jp.nintendo.com/list/software/%s.html ';
COMMENT ON COLUMN releases.l_nintendo_hk IS 'Nintendo (HK), https://store.nintendo.com.hk/%d ';
COMMENT ON COLUMN releases.l_steam IS 'Steam, https://store.steampowered.com/app/%d/ ';
COMMENT ON COLUMN releases.l_digiket IS 'Digiket, https://www.digiket.com/work/show/_data/ID=ITM%07d/ ';
COMMENT ON COLUMN releases.l_melon IS 'Melonbooks.com, https://www.melonbooks.com/index.php?main_page=product_info&products_id=IT%010d ';
COMMENT ON COLUMN releases.l_mg IS 'MangaGamer, https://www.mangagamer.com/r18/detail.php?product_code=%d ';
COMMENT ON COLUMN releases.l_getchu IS 'Getchu, http://www.getchu.com/soft.phtml?id=%d ';
COMMENT ON COLUMN releases.l_getchudl IS 'DL.Getchu, http://dl.getchu.com/i/item%d ';
COMMENT ON COLUMN releases.l_egs IS 'ErogameScape, https://erogamescape.dyndns.org/~ap2/ero/toukei_kaiseki/game.php?game=%d ';
COMMENT ON COLUMN releases.l_erotrail IS 'ErogeTrailers, http://erogetrailers.com/soft/%d (deprecated, site hasn''t been reachable for a while)';
COMMENT ON COLUMN releases.l_melonjp IS 'Melonbooks.co.jp, https://www.melonbooks.co.jp/detail/detail.php?product_id=%d ';
COMMENT ON COLUMN releases.l_gamejolt IS 'Game Jolt, https://gamejolt.com/games/vn/%d ';
COMMENT ON COLUMN releases.l_animateg IS 'Animate Games, https://www.animategames.jp/home/detail/%d ';
COMMENT ON COLUMN releases.l_freem IS 'Freem!, https://www.freem.ne.jp/win/game/%d ';
COMMENT ON COLUMN releases.l_novelgam IS 'NovelGame, https://novelgame.jp/games/show/%d ';
COMMENT ON COLUMN releases.reso_x IS 'When reso_x is 0, reso_y is either 0 for ''unknown'' or 1 for ''non-standard''.';
COMMENT ON COLUMN releases.minage IS 'Age rating, 0 - 18';
COMMENT ON COLUMN releases.ani_story IS '(old, superseded by the newer ani_* columns)';
COMMENT ON COLUMN releases.ani_ero IS '(^ but the newer columns haven''t been filled out much)';
COMMENT ON COLUMN releases.ani_story_sp IS 'Story sprite animation';
COMMENT ON COLUMN releases.ani_story_cg IS 'Story CG animation';
COMMENT ON COLUMN releases.ani_cutscene IS 'Cutscene animation';
COMMENT ON COLUMN releases.ani_ero_sp IS 'Ero scene sprite animation';
COMMENT ON COLUMN releases.ani_ero_cg IS 'Ero scene CG animation';
COMMENT ON COLUMN releases.ani_bg IS 'Background effects';
COMMENT ON COLUMN releases.ani_face IS 'Eye blink / lip sync';
COMMENT ON COLUMN releases.website IS 'Official website, %s ';
COMMENT ON COLUMN releases.l_dlsite IS 'DLsite, https://www.dlsite.com/home/work/=/product_id/%s.html ';
COMMENT ON COLUMN releases.l_gog IS 'GOG, https://www.gog.com/game/%s ';
COMMENT ON COLUMN releases.l_denpa IS 'Denpasoft, https://denpasoft.com/product/%s/ ';
COMMENT ON COLUMN releases.l_jlist IS 'J-List, https://www.jlist.com/shop/product/%s ';
COMMENT ON COLUMN releases.l_jastusa IS 'JAST USA, https://jastusa.com/games/%s/vndb ';
COMMENT ON COLUMN releases.l_itch IS 'Itch.io, https://%s ';
COMMENT ON COLUMN releases.l_nutaku IS 'Nutaku, https://www.nutaku.net/games/%s/ ';
COMMENT ON COLUMN releases.l_googplay IS 'Google Play, https://play.google.com/store/apps/details?id=%s ';
COMMENT ON COLUMN releases.l_fakku IS 'Fakku, https://www.fakku.net/games/%s ';
COMMENT ON COLUMN releases.l_freegame IS 'Freegame Mugen, https://freegame-mugen.jp/%s.html ';
COMMENT ON COLUMN releases.l_playstation_jp IS 'PlayStation Store (JP), https://store.playstation.com/ja-jp/product/%s ';
COMMENT ON COLUMN releases.l_playstation_na IS 'PlayStation Store (NA), https://store.playstation.com/en-us/product/%s ';
COMMENT ON COLUMN releases.l_playstation_eu IS 'PlayStation Store (EU), https://store.playstation.com/en-gb/product/%s ';
COMMENT ON COLUMN releases.l_playstation_hk IS 'PlayStation Store (HK), https://store.playstation.com/en-hk/product/%s ';
COMMENT ON COLUMN releases.l_nintendo IS 'Nintendo, https://www.nintendo.com/store/products/%s/ ';
COMMENT ON COLUMN releases.l_gyutto IS 'Gyutto, https://gyutto.com/i/item%d ';
COMMENT ON COLUMN releases.l_dmm IS 'DMM, https://%s ';
COMMENT ON COLUMN releases.l_booth IS 'BOOTH, https://booth.pm/en/items/%d ';
COMMENT ON COLUMN releases.l_patreonp IS 'Patreon post, https://www.patreon.com/posts/%d ';
COMMENT ON COLUMN releases.l_patreon IS 'Patreon, https://www.patreon.com/%s ';
COMMENT ON COLUMN releases.l_substar IS 'SubscribeStar, https://subscribestar.%s ';
COMMENT ON TABLE rlists IS 'User''s releases list';
COMMENT ON COLUMN rlists.status IS '0 = Unknown, 1 = Pending, 2 = Obtained, 3 = On loan, 4 = Deleted';
COMMENT ON COLUMN staff.main IS 'Primary name for the staff entry';
COMMENT ON COLUMN staff.l_anidb IS 'AniDB, https://anidb.net/cr%s ';
COMMENT ON COLUMN staff.l_wikidata IS 'Wikidata, https://www.wikidata.org/wiki/Q%d ';
COMMENT ON COLUMN staff.l_pixiv IS 'Pixiv, https://www.pixiv.net/member.php?id=%d ';
COMMENT ON COLUMN staff.l_site IS 'Official website, %s ';
COMMENT ON COLUMN staff.l_twitter IS 'Xitter, https://twitter.com/%s ';
COMMENT ON COLUMN staff.l_vgmdb IS 'VGMdb, https://vgmdb.net/artist/%d ';
COMMENT ON COLUMN staff.l_discogs IS 'Discogs, https://www.discogs.com/artist/%d ';
COMMENT ON COLUMN staff.l_mobygames IS 'MobyGames, https://www.mobygames.com/person/%d ';
COMMENT ON COLUMN staff.l_bgmtv IS 'Bangumi, https://bgm.tv/person/%d ';
COMMENT ON COLUMN staff.l_imdb IS 'IMDb, https://www.imdb.com/name/nm%07d ';
COMMENT ON COLUMN staff.l_vndb IS 'VNDB user, https://vndb.org/%s ';
COMMENT ON COLUMN staff.l_mbrainz IS 'MusicBrainz, https://musicbrainz.org/artist/%s ';
COMMENT ON COLUMN staff.l_scloud IS 'SoundCloud, https://soundcloud.com/%s ';
COMMENT ON COLUMN staff_alias.aid IS 'Globally unique ID of this alias';
COMMENT ON COLUMN tags_vn.vote IS 'negative for downvote, 1-3 otherwise';
COMMENT ON COLUMN tags_vn.lie IS 'implies spoiler=0';
COMMENT ON COLUMN traits.gid IS 'Trait group (technically a cached column, main parent''s root trait)';
COMMENT ON COLUMN traits.gorder IS 'Group order, only used when gid IS NULL';
COMMENT ON TABLE ulist_labels IS 'User labels assigned to visual novels';
COMMENT ON COLUMN ulist_labels.id IS '0 < builtin < 10 <= custom, ids are reused';
COMMENT ON TABLE ulist_vns IS 'User''s VN lists';
COMMENT ON COLUMN ulist_vns.lastmod IS 'updated when any column in this row has changed';
COMMENT ON COLUMN ulist_vns.vote_date IS 'Not updated when the vote is changed';
COMMENT ON COLUMN ulist_vns.vote IS '0 - 100';
COMMENT ON COLUMN users.ign_votes IS 'Set when user''s votes are ignored';
COMMENT ON COLUMN users.perm_imgvote IS 'User''s image votes don''t count when false';
COMMENT ON COLUMN users.perm_tag IS 'User''s tag votes don''t count when false';
COMMENT ON COLUMN users.perm_lengthvote IS 'User''s length votes don''t count when false';
COMMENT ON COLUMN vn.olang IS 'Original language';
COMMENT ON COLUMN vn.image IS 'deprecated, replaced with c_image';
COMMENT ON COLUMN vn.l_wikidata IS 'Wikidata, https://www.wikidata.org/wiki/Q%d ';
COMMENT ON COLUMN vn.c_rating IS 'decimal vote*100, i.e. 100 - 1000';
COMMENT ON COLUMN vn.c_average IS 'decimal vote*100, i.e. 100 - 1000';
COMMENT ON COLUMN vn.length IS 'Old length field, 0 = unknown, 1 = very short [..] 5 = very long';
COMMENT ON COLUMN vn.devstatus IS '0 = finished, 1 = ongoing, 2 = cancelled';
COMMENT ON COLUMN vn.l_renai IS 'Renai.us, https://renai.us/game/%s Renai.us identifier';
COMMENT ON COLUMN vn_editions.eid IS 'Edition identifier, local to the VN, not stable across revisions';
COMMENT ON COLUMN vn_length_votes.length IS 'minutes';
COMMENT ON COLUMN vn_length_votes.speed IS 'NULL=uncounted/ignored, 0=slow, 1=normal, 2=fast';
COMMENT ON TABLE wikidata IS 'Information fetched from Wikidata';
COMMENT ON COLUMN wikidata.id IS 'Q-number';
COMMENT ON COLUMN wikidata.website IS 'P856';
COMMENT ON COLUMN wikidata.vndb IS 'P3180';
COMMENT ON COLUMN wikidata.mobygames IS 'P1933';
COMMENT ON COLUMN wikidata.mobygames_company IS 'P4773';
COMMENT ON COLUMN wikidata.gamefaqs_game IS 'P4769';
COMMENT ON COLUMN wikidata.gamefaqs_company IS 'P6182';
COMMENT ON COLUMN wikidata.anidb_anime IS 'P5646';
COMMENT ON COLUMN wikidata.anidb_person IS 'P5649';
COMMENT ON COLUMN wikidata.ann_anime IS 'P1985';
COMMENT ON COLUMN wikidata.ann_manga IS 'P1984';
COMMENT ON COLUMN wikidata.musicbrainz_artist IS 'P434';
COMMENT ON COLUMN wikidata.twitter IS 'P2002';
COMMENT ON COLUMN wikidata.vgmdb_product IS 'P5659';
COMMENT ON COLUMN wikidata.vgmdb_artist IS 'P3435';
COMMENT ON COLUMN wikidata.discogs_artist IS 'P1953';
COMMENT ON COLUMN wikidata.acdb_char IS 'P7013';
COMMENT ON COLUMN wikidata.acdb_source IS 'P7017';
COMMENT ON COLUMN wikidata.indiedb_game IS 'P6717';
COMMENT ON COLUMN wikidata.howlongtobeat IS 'P2816';
COMMENT ON COLUMN wikidata.crunchyroll IS 'P4110';
COMMENT ON COLUMN wikidata.igdb_game IS 'P5794';
COMMENT ON COLUMN wikidata.giantbomb IS 'P5247';
COMMENT ON COLUMN wikidata.pcgamingwiki IS 'P6337';
COMMENT ON COLUMN wikidata.steam IS 'P1733';
COMMENT ON COLUMN wikidata.gog IS 'P2725';
COMMENT ON COLUMN wikidata.pixiv_user IS 'P5435';
COMMENT ON COLUMN wikidata.doujinshi_author IS 'P7511';
COMMENT ON COLUMN wikidata.soundcloud IS 'P3040';
COMMENT ON COLUMN wikidata.humblestore IS 'P4477';
COMMENT ON COLUMN wikidata.itchio IS 'P7294';
COMMENT ON COLUMN wikidata.playstation_jp IS 'P5999';
COMMENT ON COLUMN wikidata.playstation_na IS 'P5944';
COMMENT ON COLUMN wikidata.playstation_eu IS 'P5971';
COMMENT ON COLUMN wikidata.lutris IS 'P7597';
COMMENT ON COLUMN wikidata.wine IS 'P600';
