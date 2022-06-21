package net.insprill.customcarmanager.ui.dialog;

import javafx.scene.control.Alert;
import javafx.scene.control.Button;
import javafx.scene.control.ButtonType;
import javafx.stage.StageStyle;

import java.util.Optional;

public class ConfirmationDialog extends Alert {

    private ConfirmationDialog(String message) {
        super(AlertType.CONFIRMATION, null, ButtonType.YES, ButtonType.NO);
        initStyle(StageStyle.UTILITY);
        headerTextProperty().setValue(null);
        setContentText(message);
    }

    public static boolean show(String message) {
        return ConfirmationDialog.show(message, false);
    }

    public static boolean show(String message, boolean isYesDefault) {
        ConfirmationDialog dialog = new ConfirmationDialog(message);
        ((Button) dialog.getDialogPane().lookupButton(ButtonType.YES)).setDefaultButton(isYesDefault);
        ((Button) dialog.getDialogPane().lookupButton(ButtonType.NO)).setDefaultButton(!isYesDefault);
        Optional<ButtonType> result = dialog.showAndWait();
        return result.filter(buttonType -> buttonType == ButtonType.YES).isPresent();
    }

}
