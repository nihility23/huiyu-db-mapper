/*
 Navicat Premium Data Transfer

 Source Server         : huiyu
 Source Server Type    : SQLite
 Source Server Version : 3035005 (3.35.5)
 Source Schema         : main

 Target Server Type    : SQLite
 Target Server Version : 3035005 (3.35.5)
 File Encoding         : 65001

 Date: 25/03/2026 10:26:46
*/

PRAGMA foreign_keys = false;

-- ----------------------------
-- Table structure for sqlite_sequence
-- ----------------------------
DROP TABLE IF EXISTS "sqlite_sequence";
CREATE TABLE "sqlite_sequence" (
  "name",
  "seq"
);

-- ----------------------------
-- Records of sqlite_sequence
-- ----------------------------
INSERT INTO "sqlite_sequence" VALUES ('t_user', 2222);

-- ----------------------------
-- Table structure for t_permission
-- ----------------------------
DROP TABLE IF EXISTS "t_permission";
CREATE TABLE "t_permission" (
  "id" VARCHAR(50),
  "perm_name" VARCHAR(50) NOT NULL,
  "perm_code" VARCHAR(50) NOT NULL,
  "perm_type" TINYINT DEFAULT 1,
  "parent_id" VARCHAR(50),
  "path" VARCHAR(200),
  "component" VARCHAR(100),
  "icon" VARCHAR(50),
  "sort_order" INTEGER DEFAULT 0,
  "visible" BOOLEAN DEFAULT 1,
  "create_time" DATETIME DEFAULT CURRENT_TIMESTAMP,
  "update_time" DATETIME,
  PRIMARY KEY ("id"),
  FOREIGN KEY ("parent_id") REFERENCES "t_permission" ("id") ON DELETE CASCADE ON UPDATE NO ACTION,
  UNIQUE ("perm_code" ASC)
);

-- ----------------------------
-- Records of t_permission
-- ----------------------------
INSERT INTO "t_permission" VALUES ('perm_001', '系统管理', 'system:manage', 1, NULL, '/system', 'Layout', 'system', 1, 1, '2026-03-13 07:24:08', NULL);
INSERT INTO "t_permission" VALUES ('perm_002', '用户管理', 'user:manage', 1, 'perm_001', '/user', 'user/index', 'user', 1, 1, '2026-03-13 07:24:08', NULL);
INSERT INTO "t_permission" VALUES ('perm_003', '添加用户', 'user:add', 2, 'perm_002', NULL, NULL, 'add', 1, 1, '2026-03-13 07:24:08', NULL);
INSERT INTO "t_permission" VALUES ('perm_004', '编辑用户', 'user:edit', 2, 'perm_002', NULL, NULL, 'edit', 2, 1, '2026-03-13 07:24:08', NULL);
INSERT INTO "t_permission" VALUES ('perm_005', '删除用户', 'user:delete', 2, 'perm_002', NULL, NULL, 'delete', 3, 1, '2026-03-13 07:24:08', NULL);
INSERT INTO "t_permission" VALUES ('perm_006', '角色管理', 'role:manage', 1, 'perm_001', '/role', 'role/index', 'role', 2, 1, '2026-03-13 07:24:08', NULL);
INSERT INTO "t_permission" VALUES ('perm_007', '分配权限', 'role:assign', 3, 'perm_006', NULL, NULL, 'assign', 1, 1, '2026-03-13 07:24:08', NULL);

-- ----------------------------
-- Table structure for t_role
-- ----------------------------
DROP TABLE IF EXISTS "t_role";
CREATE TABLE "t_role" (
  "id" VARCHAR(50),
  "role_name" VARCHAR(50) NOT NULL,
  "role_code" VARCHAR(50) NOT NULL,
  "description" VARCHAR(200),
  "sort_order" INTEGER DEFAULT 0,
  "status" TINYINT DEFAULT 1,
  "is_system" BOOLEAN DEFAULT 0,
  "create_time" DATETIME DEFAULT CURRENT_TIMESTAMP,
  "update_time" DATETIME,
  PRIMARY KEY ("id"),
  UNIQUE ("role_code" ASC)
);

-- ----------------------------
-- Records of t_role
-- ----------------------------
INSERT INTO "t_role" VALUES ('role_001', '超级管理员', 'super_admin', '系统超级管理员', 1, 1, 1, '2026-03-13 07:24:08', NULL);
INSERT INTO "t_role" VALUES ('role_002', '普通管理员', 'admin', '系统普通管理员', 2, 1, 0, '2026-03-13 07:24:08', NULL);
INSERT INTO "t_role" VALUES ('role_003', '普通用户', 'user', '系统普通用户', 3, 1, 0, '2026-03-13 07:24:08', NULL);

-- ----------------------------
-- Table structure for t_role_permission
-- ----------------------------
DROP TABLE IF EXISTS "t_role_permission";
CREATE TABLE "t_role_permission" (
  "id" VARCHAR(50),
  "role_id" VARCHAR(50) NOT NULL,
  "permission_id" VARCHAR(50) NOT NULL,
  "create_time" DATETIME DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("id"),
  FOREIGN KEY ("role_id") REFERENCES "t_role" ("id") ON DELETE CASCADE ON UPDATE NO ACTION,
  FOREIGN KEY ("permission_id") REFERENCES "t_permission" ("id") ON DELETE CASCADE ON UPDATE NO ACTION,
  UNIQUE ("role_id" ASC, "permission_id" ASC)
);

-- ----------------------------
-- Records of t_role_permission
-- ----------------------------
INSERT INTO "t_role_permission" VALUES ('rp_001', 'role_001', 'perm_001', '2026-03-13 07:24:08');
INSERT INTO "t_role_permission" VALUES ('rp_002', 'role_001', 'perm_002', '2026-03-13 07:24:08');
INSERT INTO "t_role_permission" VALUES ('rp_003', 'role_001', 'perm_003', '2026-03-13 07:24:08');
INSERT INTO "t_role_permission" VALUES ('rp_004', 'role_001', 'perm_004', '2026-03-13 07:24:08');
INSERT INTO "t_role_permission" VALUES ('rp_005', 'role_001', 'perm_005', '2026-03-13 07:24:08');
INSERT INTO "t_role_permission" VALUES ('rp_006', 'role_001', 'perm_006', '2026-03-13 07:24:08');
INSERT INTO "t_role_permission" VALUES ('rp_007', 'role_001', 'perm_007', '2026-03-13 07:24:08');
INSERT INTO "t_role_permission" VALUES ('rp_008', 'role_002', 'perm_002', '2026-03-13 07:24:08');
INSERT INTO "t_role_permission" VALUES ('rp_009', 'role_002', 'perm_006', '2026-03-13 07:24:08');
INSERT INTO "t_role_permission" VALUES ('rp_010', 'role_003', 'perm_002', '2026-03-13 07:24:08');

-- ----------------------------
-- Table structure for t_user
-- ----------------------------
DROP TABLE IF EXISTS "t_user";
CREATE TABLE "t_user" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "username" VARCHAR(50) NOT NULL,
  "password" VARCHAR(100) NOT NULL,
  "real_name" VARCHAR(50),
  "email" VARCHAR(100),
  "phone" VARCHAR(20),
  "gender" TINYINT DEFAULT 0,
  "birthday" DATE,
  "height" DECIMAL(5,2),
  "is_active" BOOLEAN DEFAULT 1,
  "login_count" INTEGER DEFAULT 0,
  "last_login_time" DATETIME,
  "profile" TEXT,
  "create_time" DATETIME DEFAULT CURRENT_TIMESTAMP,
  "update_time" DATETIME,
  UNIQUE ("username" ASC)
);

-- ----------------------------
-- Records of t_user
-- ----------------------------
INSERT INTO "t_user" VALUES (1, 'admin', 'e10adc3949ba59abbe56e057f20f883e', '管理员', 'admin@example.com', '13800138000', 1, '1990-01-01', 175.5, 1, 0, NULL, '系统管理员', '2026-03-13 07:24:08', NULL);
INSERT INTO "t_user" VALUES (2, 'zhangsan', 'e10adc3949ba59abbe56e057f20f883e', '张三', 'zhangsan@example.com', '13800138001', 1, '1995-05-20', 172.3, 1, 0, NULL, '普通用户张三', '2026-03-13 07:24:08', NULL);
INSERT INTO "t_user" VALUES (3, 'lisi', 'e10adc3949ba59abbe56e057f20f883e', '李四', 'lisi@example.com', '13800138002', 2, '1998-12-10', 165.8, 0, 0, NULL, '测试用户李四', '2026-03-13 07:24:08', NULL);
INSERT INTO "t_user" VALUES (5, 'test', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:07:26', NULL);
INSERT INTO "t_user" VALUES (6, 'test_1773389666', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:14:26', NULL);
INSERT INTO "t_user" VALUES (7, 'test_1773389685', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:14:45', NULL);
INSERT INTO "t_user" VALUES (8, 'test_1773389854', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:17:34', NULL);
INSERT INTO "t_user" VALUES (9, 'test_1773389895', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:18:15', NULL);
INSERT INTO "t_user" VALUES (10, 'test_1773391703', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:48:23', NULL);
INSERT INTO "t_user" VALUES (11, 'test_1773391869', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:51:09', NULL);
INSERT INTO "t_user" VALUES (12, 'test_1773391912', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:51:52', NULL);
INSERT INTO "t_user" VALUES (13, 'test_1773391984', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:53:04', NULL);
INSERT INTO "t_user" VALUES (14, 'test_1773392044', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:54:04', NULL);
INSERT INTO "t_user" VALUES (15, 'test_1773392055', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:54:15', NULL);
INSERT INTO "t_user" VALUES (16, 'test_1773392071', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 16:54:31', NULL);
INSERT INTO "t_user" VALUES (17, 'test_1773395972', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 17:59:32', NULL);
INSERT INTO "t_user" VALUES (18, 'test_1773396861', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 18:14:21', NULL);
INSERT INTO "t_user" VALUES (19, 'test_1773397022', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 18:17:02', NULL);
INSERT INTO "t_user" VALUES (20, 'test_1773397040', '123456', 'Test User', 'test@example.com', '12345678901', 1, '2000-01-01 00:00:00', 175, 1, 0, NULL, 'This is a test profile.', '2026-03-13 18:17:20', NULL);
INSERT INTO "t_user" VALUES (2222, 'zhangsan1', 'e10adc3949ba59abbe56e057f20f883e', '张三1', 'zhangsan@example.com', '13800138001', 1, NULL, 172.300003051758, 1, 0, NULL, '普通用户张三', '2026-03-19 08:54:25', NULL);

-- ----------------------------
-- Table structure for t_user_role
-- ----------------------------
DROP TABLE IF EXISTS "t_user_role";
CREATE TABLE "t_user_role" (
  "id" VARCHAR(50),
  "user_id" INTEGER NOT NULL,
  "role_id" VARCHAR(50) NOT NULL,
  "create_time" DATETIME DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("id"),
  FOREIGN KEY ("user_id") REFERENCES "t_user" ("id") ON DELETE CASCADE ON UPDATE NO ACTION,
  FOREIGN KEY ("role_id") REFERENCES "t_role" ("id") ON DELETE CASCADE ON UPDATE NO ACTION,
  UNIQUE ("user_id" ASC, "role_id" ASC)
);

-- ----------------------------
-- Records of t_user_role
-- ----------------------------
INSERT INTO "t_user_role" VALUES ('ur_001', 1, 'role_001', '2026-03-13 07:24:08');
INSERT INTO "t_user_role" VALUES ('ur_002', 2, 'role_003', '2026-03-13 07:24:08');
INSERT INTO "t_user_role" VALUES ('ur_003', 3, 'role_003', '2026-03-13 07:24:08');
INSERT INTO "t_user_role" VALUES ('1', 13, 'role_001', '2026-03-13 18:34:14');
INSERT INTO "t_user_role" VALUES ('12', 13, 'role_002', '2026-03-13 18:34:14');

-- ----------------------------
-- Indexes structure for table t_permission
-- ----------------------------
CREATE INDEX "idx_permission_code"
ON "t_permission" (
  "perm_code" ASC
);
CREATE INDEX "idx_permission_parent"
ON "t_permission" (
  "parent_id" ASC
);

-- ----------------------------
-- Indexes structure for table t_role
-- ----------------------------
CREATE INDEX "idx_role_code"
ON "t_role" (
  "role_code" ASC
);

-- ----------------------------
-- Indexes structure for table t_role_permission
-- ----------------------------
CREATE INDEX "idx_role_perm_perm"
ON "t_role_permission" (
  "permission_id" ASC
);
CREATE INDEX "idx_role_perm_role"
ON "t_role_permission" (
  "role_id" ASC
);

-- ----------------------------
-- Auto increment value for t_user
-- ----------------------------
UPDATE "sqlite_sequence" SET seq = 2222 WHERE name = 't_user';

-- ----------------------------
-- Indexes structure for table t_user
-- ----------------------------
CREATE INDEX "idx_user_email"
ON "t_user" (
  "email" ASC
);
CREATE INDEX "idx_user_username"
ON "t_user" (
  "username" ASC
);

-- ----------------------------
-- Indexes structure for table t_user_role
-- ----------------------------
CREATE INDEX "idx_user_role_role"
ON "t_user_role" (
  "role_id" ASC
);
CREATE INDEX "idx_user_role_user"
ON "t_user_role" (
  "user_id" ASC
);

PRAGMA foreign_keys = true;
