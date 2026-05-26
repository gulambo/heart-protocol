import React, { useState } from 'react';
import { View, Text, StyleSheet, TouchableOpacity, Vibration } from 'react-native';

export default function HeartProtocolDashboard() {
  const [alertActive, setAlertActive] = useState(true); 

  const handleBiometricHandshake = () => {
    Vibration.vibrate([0, 500, 200, 500]); // Firm tactile engagement pulse
    console.log("Generating local zero-knowledge proof of vitality...");
    setAlertActive(false); // Hardware safety gate unlocked
  };

  return (
    <View style={[styles.container, alertActive ? styles.bgAlert : styles.bgNormal]}>
      {alertActive ? (
        <View style={styles.alertCard}>
          <Text style={styles.warningText}>⚠️ KINETIC EXCEPTION</Text>
          <Text style={styles.unitText}>Unit 04: Boundary Breach Detected</Text>
          <Text style={styles.detailText}>Physical movement locked. Awaiting biometric authorization.</Text>
          
          <TouchableOpacity 
            style={styles.pohButton} 
            onLongPress={handleBiometricHandshake}
            delayLongPress={1500}
          >
            <Text style={styles.buttonText}>HOLD TO AUTHORIZE (PoH)</Text>
          </TouchableOpacity>
        </View>
      ) : (
        <View style={styles.idleState}>
          <Text style={styles.missionHeader}>HeartProtocol</Text>
          <Text style={styles.missionText}>
            Every autonomous machine that shares our world answers to a cryptographically verified human heart.
          </Text>
          <Text style={styles.statusText}>● All registered network units secure.</Text>
        </View>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: 'center', padding: 20 },
  bgAlert: { backgroundColor: '#4A0000' }, 
  bgNormal: { backgroundColor: '#050505' }, 
  alertCard: { backgroundColor: '#1A0000', padding: 30, borderRadius: 15, borderColor: '#FF3333', borderWidth: 2 },
  warningText: { color: '#FF3333', fontSize: 24, fontWeight: 'bold', marginBottom: 10 },
  unitText: { color: 'white', fontSize: 18, marginBottom: 5 },
  detailText: { color: '#AAAAAA', marginBottom: 30 },
  pohButton: { backgroundColor: '#FF3333', padding: 20, borderRadius: 50, alignItems: 'center' },
  buttonText: { color: 'white', fontWeight: 'bold', fontSize: 16, letterSpacing: 2 },
  idleState: { padding: 20 },
  missionHeader: { color: 'white', fontSize: 32, fontWeight: '900', marginBottom: 10 },
  missionText: { color: '#888888', fontSize: 18, lineHeight: 26, marginBottom: 40 },
  statusText: { color: '#00FF00', fontSize: 16, fontWeight: '600' }
});
