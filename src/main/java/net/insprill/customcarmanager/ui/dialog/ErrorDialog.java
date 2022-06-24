package net.insprill.customcarmanager.ui.dialog;

import javafx.event.ActionEvent;
import javafx.scene.control.Alert;
import javafx.scene.control.ButtonBar;
import javafx.scene.control.ButtonType;
import javafx.scene.control.TextArea;
import javafx.scene.layout.Region;
import javafx.scene.layout.VBox;
import javafx.scene.text.Text;
import javafx.stage.StageStyle;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.util.TextHelper;

public class ErrorDialog extends Alert {

    private final VBox content = new VBox();

    private ErrorDialog() {
        super(AlertType.ERROR, null, ButtonType.CLOSE);
        initStyle(StageStyle.UTILITY);
        headerTextProperty().setValue(Locale.getLine("dialog.error.title"));
    }

    private void setMessage(String message) {
        content.getChildren().add(new Text(message));
    }

    private void setException(Throwable ex) {
        String errorMessage = TextHelper.getStacktrace(ex);

        ButtonType copyToClipboard = new ButtonType(Locale.getLine("dialog.error.copy-stacktrace-to-clipboard"), ButtonBar.ButtonData.BACK_PREVIOUS);
        getDialogPane().getButtonTypes().add(0, copyToClipboard);

        TextArea errorText = new TextArea(errorMessage);
        errorText.setEditable(false);

        getDialogPane().lookupButton(copyToClipboard).addEventFilter(ActionEvent.ACTION, e -> {
            TextHelper.copyToClipboard(errorMessage);
            e.consume();
        });

        content.getChildren().add(errorText);
    }

    public static void show(String message) {
        show(message, null);
    }

    public static void show(Throwable ex) {
        show(null, ex);
    }

    public static void show(String message, Throwable ex) {
        ErrorDialog dialog = new ErrorDialog();

        if (message != null) {
            if (ex != null) {
                dialog.setMessage(message);
            } else {
                dialog.setContentText(message);
            }
        }

        if (ex != null) {
            if (message != null) {
                Region spacer = new Region();
                spacer.setPrefHeight(10);
                dialog.content.getChildren().add(spacer);
            }
            dialog.setException(ex);
            dialog.getDialogPane().setContent(dialog.content);
        }

        dialog.showAndWait();
    }

}
