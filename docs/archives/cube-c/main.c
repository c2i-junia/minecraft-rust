#include "raylib.h"
#include <math.h>

int main(void)
{
    // Dimensions de la fenêtre
    const int screenWidth = 800;
    const int screenHeight = 600;

    // Initialisation de la fenêtre
    InitWindow(screenWidth, screenHeight, "Mini-Jeu 3D - Contrôle de la caméra");

    // Configuration de la caméra
    Camera camera = { 0 };
    camera.up = (Vector3){ 0.0f, 1.0f, 0.0f };
    camera.fovy = 45.0f;
    camera.projection = CAMERA_PERSPECTIVE;

    SetTargetFPS(60); // Limite à 60 FPS

    // Position du joueur
    Vector3 playerPosition = { 0.0f, 1.0f, 0.0f };
    float playerSize = 1.0f;

    // Taille de la plateforme
    Vector3 platformSize = { 20.0f, 1.0f, 20.0f };

    // Vitesse de déplacement
    float playerSpeed = 5.0f;

    // Variables pour le contrôle de la caméra
    float cameraDistance = 10.0f; // Distance entre la caméra et le joueur
    float cameraAngleX = 0.0f;    // Angle horizontal (autour de l'axe Y)
    float cameraAngleY = 20.0f * DEG2RAD;   // Angle vertical (en radians)

    // Sensibilité de la souris
    float mouseSensitivity = 0.003f;

    // Cache le curseur
    DisableCursor();

    // Boucle principale du jeu
    while (!WindowShouldClose())
    {
        // Récupération du déplacement de la souris
        Vector2 mouseDelta = GetMouseDelta();

        // Mise à jour des angles de la caméra
        cameraAngleX -= mouseDelta.x * mouseSensitivity;
        cameraAngleY += mouseDelta.y * mouseSensitivity;

        // Limiter l'angle vertical pour éviter les rotations complètes
        if (cameraAngleY > 89.0f * DEG2RAD) cameraAngleY = 89.0f * DEG2RAD;
        if (cameraAngleY < -89.0f * DEG2RAD) cameraAngleY = -89.0f * DEG2RAD;

        // Calcul de la position de la caméra
        camera.position.x = playerPosition.x + cameraDistance * cosf(cameraAngleY) * sinf(cameraAngleX);
        camera.position.y = playerPosition.y + cameraDistance * sinf(cameraAngleY);
        camera.position.z = playerPosition.z + cameraDistance * cosf(cameraAngleY) * cosf(cameraAngleX);

        // La caméra regarde le joueur
        camera.target = playerPosition;

        // Direction de déplacement
        Vector3 forward = {
            sinf(cameraAngleX),
            0.0f,
            cosf(cameraAngleX)
        };

        Vector3 right = {
            cosf(cameraAngleX),
            0.0f,
            -sinf(cameraAngleX)
        };

        // Gestion des entrées clavier pour le déplacement du joueur
        if (IsKeyDown(KEY_S)) {
            playerPosition.x += forward.x * playerSpeed * GetFrameTime();
            playerPosition.z += forward.z * playerSpeed * GetFrameTime();
        }
        if (IsKeyDown(KEY_W)) {
            playerPosition.x -= forward.x * playerSpeed * GetFrameTime();
            playerPosition.z -= forward.z * playerSpeed * GetFrameTime();
        }
        if (IsKeyDown(KEY_A)) {
            playerPosition.x -= right.x * playerSpeed * GetFrameTime();
            playerPosition.z -= right.z * playerSpeed * GetFrameTime();
        }
        if (IsKeyDown(KEY_D)) {
            playerPosition.x += right.x * playerSpeed * GetFrameTime();
            playerPosition.z += right.z * playerSpeed * GetFrameTime();
        }

        BeginDrawing();

            ClearBackground(RAYWHITE);

            BeginMode3D(camera);

                // Dessiner la plateforme
                DrawCube((Vector3){0.0f, 0.0f, 0.0f}, platformSize.x, platformSize.y, platformSize.z, LIGHTGRAY);
                DrawCubeWires((Vector3){0.0f, 0.0f, 0.0f}, platformSize.x, platformSize.y, platformSize.z, DARKGRAY);

                // Dessiner le joueur
                DrawCube(playerPosition, playerSize, playerSize, playerSize, RED);
                DrawCubeWires(playerPosition, playerSize, playerSize, playerSize, MAROON);

            EndMode3D();

            DrawFPS(10, 10);

        EndDrawing();
    }

    // Affiche le curseur avant de quitter
    EnableCursor();

    // Déchargement des ressources
    CloseWindow();

    return 0;
}


