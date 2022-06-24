package net.insprill.customcarmanager.ui.dialog;

import javafx.scene.control.Alert;
import javafx.scene.control.ButtonType;
import javafx.stage.StageStyle;

public class InfoDialog extends Alert {

    private InfoDialog() {
        super(AlertType.INFORMATION, null, ButtonType.CLOSE);
        initStyle(StageStyle.UTILITY);
        headerTextProperty().setValue(null);
    }

    public static void show(String message) {
        show(message, false);
    }

    public static void show(String message, boolean blocking) {
        InfoDialog dialog = new InfoDialog();

        dialog.setContentText(message);

        if (blocking) {
            dialog.showAndWait();
        } else {
            dialog.show();
        }
    }

}
