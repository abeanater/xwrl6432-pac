#[doc = "Register `CRC_CTRL0` reader"]
pub type R = crate::R<CrcCtrl0Spec>;
#[doc = "Register `CRC_CTRL0` writer"]
pub type W = crate::W<CrcCtrl0Spec>;
#[doc = "Field `CH1_PSA_SWREST` reader - 0:0\\]
Channel 1 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a ΓÇÿ0ΓÇÖ. 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch1PsaSwrestR = crate::BitReader;
#[doc = "Field `CH1_PSA_SWREST` writer - 0:0\\]
Channel 1 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a ΓÇÿ0ΓÇÖ. 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch1PsaSwrestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_DW_SEL` reader - 2:1\\]
CRC Data Size select. 000 ΓÇô Not Supported 001 - 16 bit Data Size 010 ΓÇô 32 Bit Data Size"]
pub type Ch1DwSelR = crate::FieldReader;
#[doc = "Field `CH1_DW_SEL` writer - 2:1\\]
CRC Data Size select. 000 ΓÇô Not Supported 001 - 16 bit Data Size 010 ΓÇô 32 Bit Data Size"]
pub type Ch1DwSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_CRC_SEL` reader - 4:3\\]
CRC type select. {CH1_CRC_SEL2,CH1_CRC_SEL\\[1:0\\]} 000 ΓÇô CRC-64 001 - CRC-16 010 ΓÇô CRC-32 100 - VDA CAN, SAE-J1850 CRC-8 101 - H2F, Autosar 4.0 110 - CASTAGNOLI, iSCSI 111 / 011 - E2E Profile 4"]
pub type Ch1CrcSelR = crate::FieldReader;
#[doc = "Field `CH1_CRC_SEL` writer - 4:3\\]
CRC type select. {CH1_CRC_SEL2,CH1_CRC_SEL\\[1:0\\]} 000 ΓÇô CRC-64 001 - CRC-16 010 ΓÇô CRC-32 100 - VDA CAN, SAE-J1850 CRC-8 101 - H2F, Autosar 4.0 110 - CASTAGNOLI, iSCSI 111 / 011 - E2E Profile 4"]
pub type Ch1CrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_BIT_SWAP` reader - 5:5\\]
msb/lsb SWAPPING 0 ΓÇô msb (most significant bit First) 1 ΓÇô lsb (least significant bit First)"]
pub type Ch1BitSwapR = crate::BitReader;
#[doc = "Field `CH1_BIT_SWAP` writer - 5:5\\]
msb/lsb SWAPPING 0 ΓÇô msb (most significant bit First) 1 ΓÇô lsb (least significant bit First)"]
pub type Ch1BitSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_BYTE_SWAP` reader - 6:6\\]
BYTE SWAP Enable across Data Size 0 ΓÇô Byte Swap Disabled 1 ΓÇô Byte Swap enabled."]
pub type Ch1ByteSwapR = crate::BitReader;
#[doc = "Field `CH1_BYTE_SWAP` writer - 6:6\\]
BYTE SWAP Enable across Data Size 0 ΓÇô Byte Swap Disabled 1 ΓÇô Byte Swap enabled."]
pub type Ch1ByteSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CRC_SEL2` reader - 7:7\\]
Refer \"CH1_DW_SEL\" field description"]
pub type Ch1CrcSel2R = crate::BitReader;
#[doc = "Field `CH1_CRC_SEL2` writer - 7:7\\]
Refer \"CH1_DW_SEL\" field description"]
pub type Ch1CrcSel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_PSA_SWREST` reader - 8:8\\]
Channel 2 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a ΓÇÿ0ΓÇÖ. 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch2PsaSwrestR = crate::BitReader;
#[doc = "Field `CH2_PSA_SWREST` writer - 8:8\\]
Channel 2 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a ΓÇÿ0ΓÇÖ. 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
pub type Ch2PsaSwrestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_DW_SEL` reader - 10:9\\]
CRC Data Size select. 000 ΓÇô Not Supported 001 - 16 bit Data Size 010 ΓÇô 32 Bit Data Size"]
pub type Ch2DwSelR = crate::FieldReader;
#[doc = "Field `CH2_DW_SEL` writer - 10:9\\]
CRC Data Size select. 000 ΓÇô Not Supported 001 - 16 bit Data Size 010 ΓÇô 32 Bit Data Size"]
pub type Ch2DwSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2_CRC_SEL` reader - 12:11\\]
CRC type select. {CH1_CRC_SEL2,CH1_CRC_SEL\\[1:0\\]} 000 ΓÇô CRC-64 001 - CRC-16 010 ΓÇô CRC-32 100 - VDA CAN, SAE-J1850 CRC-8 101 - H2F, Autosar 4.0 110 - CASTAGNOLI, iSCSI 111 / 011 - E2E Profile 4"]
pub type Ch2CrcSelR = crate::FieldReader;
#[doc = "Field `CH2_CRC_SEL` writer - 12:11\\]
CRC type select. {CH1_CRC_SEL2,CH1_CRC_SEL\\[1:0\\]} 000 ΓÇô CRC-64 001 - CRC-16 010 ΓÇô CRC-32 100 - VDA CAN, SAE-J1850 CRC-8 101 - H2F, Autosar 4.0 110 - CASTAGNOLI, iSCSI 111 / 011 - E2E Profile 4"]
pub type Ch2CrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2_BIT_SWAP` reader - 13:13\\]
msb/lsb SWAPPING 0 ΓÇô msb (most significant bit First) 1 ΓÇô lsb (least significant bit First)"]
pub type Ch2BitSwapR = crate::BitReader;
#[doc = "Field `CH2_BIT_SWAP` writer - 13:13\\]
msb/lsb SWAPPING 0 ΓÇô msb (most significant bit First) 1 ΓÇô lsb (least significant bit First)"]
pub type Ch2BitSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_BYTE_SWAP` reader - 14:14\\]
BYTE SWAP Enable across Data Size 0 ΓÇô Byte Swap Disabled 1 ΓÇô Byte Swap enabled."]
pub type Ch2ByteSwapR = crate::BitReader;
#[doc = "Field `CH2_BYTE_SWAP` writer - 14:14\\]
BYTE SWAP Enable across Data Size 0 ΓÇô Byte Swap Disabled 1 ΓÇô Byte Swap enabled."]
pub type Ch2ByteSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_CRC_SEL2` reader - 15:15\\]
Refer \"CH2_DW_SEL\" field description"]
pub type Ch2CrcSel2R = crate::BitReader;
#[doc = "Field `CH2_CRC_SEL2` writer - 15:15\\]
Refer \"CH2_DW_SEL\" field description"]
pub type Ch2CrcSel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 16:16\\]
Reserved"]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - 16:16\\]
Reserved"]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 18:17\\]
Reserved"]
pub type Nu2R = crate::FieldReader;
#[doc = "Field `NU2` writer - 18:17\\]
Reserved"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU3` reader - 20:19\\]
Reserved"]
pub type Nu3R = crate::FieldReader;
#[doc = "Field `NU3` writer - 20:19\\]
Reserved"]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU4` reader - 21:21\\]
Reserved"]
pub type Nu4R = crate::BitReader;
#[doc = "Field `NU4` writer - 21:21\\]
Reserved"]
pub type Nu4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU5` reader - 22:22\\]
Reserved"]
pub type Nu5R = crate::BitReader;
#[doc = "Field `NU5` writer - 22:22\\]
Reserved"]
pub type Nu5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU6` reader - 23:23\\]
Reserved"]
pub type Nu6R = crate::BitReader;
#[doc = "Field `NU6` writer - 23:23\\]
Reserved"]
pub type Nu6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU7` reader - 24:24\\]
Reserved"]
pub type Nu7R = crate::BitReader;
#[doc = "Field `NU7` writer - 24:24\\]
Reserved"]
pub type Nu7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU8` reader - 26:25\\]
Reserved"]
pub type Nu8R = crate::FieldReader;
#[doc = "Field `NU8` writer - 26:25\\]
Reserved"]
pub type Nu8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU9` reader - 28:27\\]
Reserved"]
pub type Nu9R = crate::FieldReader;
#[doc = "Field `NU9` writer - 28:27\\]
Reserved"]
pub type Nu9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU10` reader - 29:29\\]
Reserved"]
pub type Nu10R = crate::BitReader;
#[doc = "Field `NU10` writer - 29:29\\]
Reserved"]
pub type Nu10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU11` reader - 30:30\\]
Reserved"]
pub type Nu11R = crate::BitReader;
#[doc = "Field `NU11` writer - 30:30\\]
Reserved"]
pub type Nu11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU12` reader - 31:31\\]
Reserved"]
pub type Nu12R = crate::BitReader;
#[doc = "Field `NU12` writer - 31:31\\]
Reserved"]
pub type Nu12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel 1 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a ΓÇÿ0ΓÇÖ. 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    pub fn ch1_psa_swrest(&self) -> Ch1PsaSwrestR {
        Ch1PsaSwrestR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
CRC Data Size select. 000 ΓÇô Not Supported 001 - 16 bit Data Size 010 ΓÇô 32 Bit Data Size"]
    #[inline(always)]
    pub fn ch1_dw_sel(&self) -> Ch1DwSelR {
        Ch1DwSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
CRC type select. {CH1_CRC_SEL2,CH1_CRC_SEL\\[1:0\\]} 000 ΓÇô CRC-64 001 - CRC-16 010 ΓÇô CRC-32 100 - VDA CAN, SAE-J1850 CRC-8 101 - H2F, Autosar 4.0 110 - CASTAGNOLI, iSCSI 111 / 011 - E2E Profile 4"]
    #[inline(always)]
    pub fn ch1_crc_sel(&self) -> Ch1CrcSelR {
        Ch1CrcSelR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
msb/lsb SWAPPING 0 ΓÇô msb (most significant bit First) 1 ΓÇô lsb (least significant bit First)"]
    #[inline(always)]
    pub fn ch1_bit_swap(&self) -> Ch1BitSwapR {
        Ch1BitSwapR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
BYTE SWAP Enable across Data Size 0 ΓÇô Byte Swap Disabled 1 ΓÇô Byte Swap enabled."]
    #[inline(always)]
    pub fn ch1_byte_swap(&self) -> Ch1ByteSwapR {
        Ch1ByteSwapR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Refer \"CH1_DW_SEL\" field description"]
    #[inline(always)]
    pub fn ch1_crc_sel2(&self) -> Ch1CrcSel2R {
        Ch1CrcSel2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 2 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a ΓÇÿ0ΓÇÖ. 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    pub fn ch2_psa_swrest(&self) -> Ch2PsaSwrestR {
        Ch2PsaSwrestR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - 10:9\\]
CRC Data Size select. 000 ΓÇô Not Supported 001 - 16 bit Data Size 010 ΓÇô 32 Bit Data Size"]
    #[inline(always)]
    pub fn ch2_dw_sel(&self) -> Ch2DwSelR {
        Ch2DwSelR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
CRC type select. {CH1_CRC_SEL2,CH1_CRC_SEL\\[1:0\\]} 000 ΓÇô CRC-64 001 - CRC-16 010 ΓÇô CRC-32 100 - VDA CAN, SAE-J1850 CRC-8 101 - H2F, Autosar 4.0 110 - CASTAGNOLI, iSCSI 111 / 011 - E2E Profile 4"]
    #[inline(always)]
    pub fn ch2_crc_sel(&self) -> Ch2CrcSelR {
        Ch2CrcSelR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
msb/lsb SWAPPING 0 ΓÇô msb (most significant bit First) 1 ΓÇô lsb (least significant bit First)"]
    #[inline(always)]
    pub fn ch2_bit_swap(&self) -> Ch2BitSwapR {
        Ch2BitSwapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
BYTE SWAP Enable across Data Size 0 ΓÇô Byte Swap Disabled 1 ΓÇô Byte Swap enabled."]
    #[inline(always)]
    pub fn ch2_byte_swap(&self) -> Ch2ByteSwapR {
        Ch2ByteSwapR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Refer \"CH2_DW_SEL\" field description"]
    #[inline(always)]
    pub fn ch2_crc_sel2(&self) -> Ch2CrcSel2R {
        Ch2CrcSel2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - 18:17\\]
Reserved"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Reserved"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Reserved"]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Reserved"]
    #[inline(always)]
    pub fn nu5(&self) -> Nu5R {
        Nu5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved"]
    #[inline(always)]
    pub fn nu6(&self) -> Nu6R {
        Nu6R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Reserved"]
    #[inline(always)]
    pub fn nu7(&self) -> Nu7R {
        Nu7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Reserved"]
    #[inline(always)]
    pub fn nu8(&self) -> Nu8R {
        Nu8R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Reserved"]
    #[inline(always)]
    pub fn nu9(&self) -> Nu9R {
        Nu9R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
Reserved"]
    #[inline(always)]
    pub fn nu10(&self) -> Nu10R {
        Nu10R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Reserved"]
    #[inline(always)]
    pub fn nu11(&self) -> Nu11R {
        Nu11R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved"]
    #[inline(always)]
    pub fn nu12(&self) -> Nu12R {
        Nu12R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel 1 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a ΓÇÿ0ΓÇÖ. 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_psa_swrest(&mut self) -> Ch1PsaSwrestW<CrcCtrl0Spec> {
        Ch1PsaSwrestW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
CRC Data Size select. 000 ΓÇô Not Supported 001 - 16 bit Data Size 010 ΓÇô 32 Bit Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_dw_sel(&mut self) -> Ch1DwSelW<CrcCtrl0Spec> {
        Ch1DwSelW::new(self, 1)
    }
    #[doc = "Bits 3:4 - 4:3\\]
CRC type select. {CH1_CRC_SEL2,CH1_CRC_SEL\\[1:0\\]} 000 ΓÇô CRC-64 001 - CRC-16 010 ΓÇô CRC-32 100 - VDA CAN, SAE-J1850 CRC-8 101 - H2F, Autosar 4.0 110 - CASTAGNOLI, iSCSI 111 / 011 - E2E Profile 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_crc_sel(&mut self) -> Ch1CrcSelW<CrcCtrl0Spec> {
        Ch1CrcSelW::new(self, 3)
    }
    #[doc = "Bit 5 - 5:5\\]
msb/lsb SWAPPING 0 ΓÇô msb (most significant bit First) 1 ΓÇô lsb (least significant bit First)"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_bit_swap(&mut self) -> Ch1BitSwapW<CrcCtrl0Spec> {
        Ch1BitSwapW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
BYTE SWAP Enable across Data Size 0 ΓÇô Byte Swap Disabled 1 ΓÇô Byte Swap enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_byte_swap(&mut self) -> Ch1ByteSwapW<CrcCtrl0Spec> {
        Ch1ByteSwapW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Refer \"CH1_DW_SEL\" field description"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_crc_sel2(&mut self) -> Ch1CrcSel2W<CrcCtrl0Spec> {
        Ch1CrcSel2W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 2 PSA Software Reset. When set, the PSA Signature Register is reset to all zero. Software reset does not reset software reset bit itself. Therefore, CPU is required to clear this bit by writing a ΓÇÿ0ΓÇÖ. 0 = PSA Signature Register not reset 1 = PSA Signature Register reset"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_psa_swrest(&mut self) -> Ch2PsaSwrestW<CrcCtrl0Spec> {
        Ch2PsaSwrestW::new(self, 8)
    }
    #[doc = "Bits 9:10 - 10:9\\]
CRC Data Size select. 000 ΓÇô Not Supported 001 - 16 bit Data Size 010 ΓÇô 32 Bit Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_dw_sel(&mut self) -> Ch2DwSelW<CrcCtrl0Spec> {
        Ch2DwSelW::new(self, 9)
    }
    #[doc = "Bits 11:12 - 12:11\\]
CRC type select. {CH1_CRC_SEL2,CH1_CRC_SEL\\[1:0\\]} 000 ΓÇô CRC-64 001 - CRC-16 010 ΓÇô CRC-32 100 - VDA CAN, SAE-J1850 CRC-8 101 - H2F, Autosar 4.0 110 - CASTAGNOLI, iSCSI 111 / 011 - E2E Profile 4"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_crc_sel(&mut self) -> Ch2CrcSelW<CrcCtrl0Spec> {
        Ch2CrcSelW::new(self, 11)
    }
    #[doc = "Bit 13 - 13:13\\]
msb/lsb SWAPPING 0 ΓÇô msb (most significant bit First) 1 ΓÇô lsb (least significant bit First)"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_bit_swap(&mut self) -> Ch2BitSwapW<CrcCtrl0Spec> {
        Ch2BitSwapW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
BYTE SWAP Enable across Data Size 0 ΓÇô Byte Swap Disabled 1 ΓÇô Byte Swap enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_byte_swap(&mut self) -> Ch2ByteSwapW<CrcCtrl0Spec> {
        Ch2ByteSwapW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Refer \"CH2_DW_SEL\" field description"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_crc_sel2(&mut self) -> Ch2CrcSel2W<CrcCtrl0Spec> {
        Ch2CrcSel2W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<CrcCtrl0Spec> {
        Nu1W::new(self, 16)
    }
    #[doc = "Bits 17:18 - 18:17\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<CrcCtrl0Spec> {
        Nu2W::new(self, 17)
    }
    #[doc = "Bits 19:20 - 20:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<CrcCtrl0Spec> {
        Nu3W::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<CrcCtrl0Spec> {
        Nu4W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu5(&mut self) -> Nu5W<CrcCtrl0Spec> {
        Nu5W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu6(&mut self) -> Nu6W<CrcCtrl0Spec> {
        Nu6W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu7(&mut self) -> Nu7W<CrcCtrl0Spec> {
        Nu7W::new(self, 24)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu8(&mut self) -> Nu8W<CrcCtrl0Spec> {
        Nu8W::new(self, 25)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu9(&mut self) -> Nu9W<CrcCtrl0Spec> {
        Nu9W::new(self, 27)
    }
    #[doc = "Bit 29 - 29:29\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu10(&mut self) -> Nu10W<CrcCtrl0Spec> {
        Nu10W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu11(&mut self) -> Nu11W<CrcCtrl0Spec> {
        Nu11W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu12(&mut self) -> Nu12W<CrcCtrl0Spec> {
        Nu12W::new(self, 31)
    }
}
#[doc = "Contains sw reset control bit to reset PSA\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCtrl0Spec;
impl crate::RegisterSpec for CrcCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_ctrl0::R`](R) reader structure"]
impl crate::Readable for CrcCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_ctrl0::W`](W) writer structure"]
impl crate::Writable for CrcCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_CTRL0 to value 0"]
impl crate::Resettable for CrcCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
