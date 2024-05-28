DROP TRIGGER IF EXISTS avoid_learning_map_cycles_trigger ON learning_map_components;

DROP TABLE learning_map_components;

DROP TABLE learning_maps;

DROP TRIGGER sync_like_notif_delete ON likes;

DROP TRIGGER sync_like_notif_insert ON likes;

DROP TABLE notifications;

DROP TRIGGER IF EXISTS catch_satisfier_deletion on objective_satisfiers;

DROP TABLE objective_satisfiers;

