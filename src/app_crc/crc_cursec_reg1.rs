#[doc = "Register `CRC_CURSEC_REG1` reader"]
pub type R = crate::R<CrcCursecReg1Spec>;
#[doc = "Register `CRC_CURSEC_REG1` writer"]
pub type W = crate::W<CrcCursecReg1Spec>;
#[doc = "Field `CRC_CURSEC1` reader - 15:0\\]
Channel 1 Current Sector ID Register. In AUTO mode, this register contains the current sector number of which the signature verification fails. The sector counter is a free running up counter. When a sector fails, the erroneous sector number is logged into current sector ID register and the CRC fail interrupt is generated The sector ID register is frozen until it is read and the CRC fail status bit is cleared by CPU. While it is frozen, it does not capture another erroneous sector number. When this condition happens, an overrun interrupt is generated instead. Once the register is read and the CRC fail interrupt flag is cleared it can capture new erro- neous sector number."]
pub type CrcCursec1R = crate::FieldReader<u16>;
#[doc = "Field `CRC_CURSEC1` writer - 15:0\\]
Channel 1 Current Sector ID Register. In AUTO mode, this register contains the current sector number of which the signature verification fails. The sector counter is a free running up counter. When a sector fails, the erroneous sector number is logged into current sector ID register and the CRC fail interrupt is generated The sector ID register is frozen until it is read and the CRC fail status bit is cleared by CPU. While it is frozen, it does not capture another erroneous sector number. When this condition happens, an overrun interrupt is generated instead. Once the register is read and the CRC fail interrupt flag is cleared it can capture new erro- neous sector number."]
pub type CrcCursec1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved1` reader - 31:16\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:16\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Channel 1 Current Sector ID Register. In AUTO mode, this register contains the current sector number of which the signature verification fails. The sector counter is a free running up counter. When a sector fails, the erroneous sector number is logged into current sector ID register and the CRC fail interrupt is generated The sector ID register is frozen until it is read and the CRC fail status bit is cleared by CPU. While it is frozen, it does not capture another erroneous sector number. When this condition happens, an overrun interrupt is generated instead. Once the register is read and the CRC fail interrupt flag is cleared it can capture new erro- neous sector number."]
    #[inline(always)]
    pub fn crc_cursec1(&self) -> CrcCursec1R {
        CrcCursec1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Channel 1 Current Sector ID Register. In AUTO mode, this register contains the current sector number of which the signature verification fails. The sector counter is a free running up counter. When a sector fails, the erroneous sector number is logged into current sector ID register and the CRC fail interrupt is generated The sector ID register is frozen until it is read and the CRC fail status bit is cleared by CPU. While it is frozen, it does not capture another erroneous sector number. When this condition happens, an overrun interrupt is generated instead. Once the register is read and the CRC fail interrupt flag is cleared it can capture new erro- neous sector number."]
    #[inline(always)]
    #[must_use]
    pub fn crc_cursec1(&mut self) -> CrcCursec1W<CrcCursecReg1Spec> {
        CrcCursec1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcCursecReg1Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "Channel 1 current sector register contains the sector number which causes CRC failure\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_cursec_reg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_cursec_reg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCursecReg1Spec;
impl crate::RegisterSpec for CrcCursecReg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_cursec_reg1::R`](R) reader structure"]
impl crate::Readable for CrcCursecReg1Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_cursec_reg1::W`](W) writer structure"]
impl crate::Writable for CrcCursecReg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_CURSEC_REG1 to value 0"]
impl crate::Resettable for CrcCursecReg1Spec {
    const RESET_VALUE: u32 = 0;
}
