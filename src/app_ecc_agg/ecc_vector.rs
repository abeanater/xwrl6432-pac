#[doc = "Register `ECC_VECTOR` reader"]
pub type R = crate::R<EccVectorSpec>;
#[doc = "Register `ECC_VECTOR` writer"]
pub type W = crate::W<EccVectorSpec>;
#[doc = "Field `ECC_VECTOR` reader - 10:0\\]
Value written to select the corresponding ECC RAM for control or status - (RW )"]
pub type EccVectorR = crate::FieldReader<u16>;
#[doc = "Field `ECC_VECTOR` writer - 10:0\\]
Value written to select the corresponding ECC RAM for control or status - (RW )"]
pub type EccVectorW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RES2` reader - 14:11\\]
RESERVE FIELD"]
pub type Res2R = crate::FieldReader;
#[doc = "Field `RES2` writer - 14:11\\]
RESERVE FIELD"]
pub type Res2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RD_SVBUS` reader - 15:15\\]
Write 1 to trigger a read on the serial VBUS - (RW )"]
pub type RdSvbusR = crate::BitReader;
#[doc = "Field `RD_SVBUS` writer - 15:15\\]
Write 1 to trigger a read on the serial VBUS - (RW )"]
pub type RdSvbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_SVBUS_ADDRESS` reader - 23:16\\]
Read address - (RW )"]
pub type RdSvbusAddressR = crate::FieldReader;
#[doc = "Field `RD_SVBUS_ADDRESS` writer - 23:16\\]
Read address - (RW )"]
pub type RdSvbusAddressW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD_SVBUS_DONE` reader - 24:24\\]
Status to indicate if read on serial VBUS is complete - (RO )"]
pub type RdSvbusDoneR = crate::BitReader;
#[doc = "Field `RD_SVBUS_DONE` writer - 24:24\\]
Status to indicate if read on serial VBUS is complete - (RO )"]
pub type RdSvbusDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES1` reader - 31:25\\]
RESERVE FIELD"]
pub type Res1R = crate::FieldReader;
#[doc = "Field `RES1` writer - 31:25\\]
RESERVE FIELD"]
pub type Res1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Value written to select the corresponding ECC RAM for control or status - (RW )"]
    #[inline(always)]
    pub fn ecc_vector(&self) -> EccVectorR {
        EccVectorR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - 14:11\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res2(&self) -> Res2R {
        Res2R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to trigger a read on the serial VBUS - (RW )"]
    #[inline(always)]
    pub fn rd_svbus(&self) -> RdSvbusR {
        RdSvbusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Read address - (RW )"]
    #[inline(always)]
    pub fn rd_svbus_address(&self) -> RdSvbusAddressR {
        RdSvbusAddressR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Status to indicate if read on serial VBUS is complete - (RO )"]
    #[inline(always)]
    pub fn rd_svbus_done(&self) -> RdSvbusDoneR {
        RdSvbusDoneR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res1(&self) -> Res1R {
        Res1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Value written to select the corresponding ECC RAM for control or status - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_vector(&mut self) -> EccVectorW<EccVectorSpec> {
        EccVectorW::new(self, 0)
    }
    #[doc = "Bits 11:14 - 14:11\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res2(&mut self) -> Res2W<EccVectorSpec> {
        Res2W::new(self, 11)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to trigger a read on the serial VBUS - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn rd_svbus(&mut self) -> RdSvbusW<EccVectorSpec> {
        RdSvbusW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Read address - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn rd_svbus_address(&mut self) -> RdSvbusAddressW<EccVectorSpec> {
        RdSvbusAddressW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Status to indicate if read on serial VBUS is complete - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn rd_svbus_done(&mut self) -> RdSvbusDoneW<EccVectorSpec> {
        RdSvbusDoneW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res1(&mut self) -> Res1W<EccVectorSpec> {
        Res1W::new(self, 25)
    }
}
#[doc = "ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_vector::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_vector::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccVectorSpec;
impl crate::RegisterSpec for EccVectorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_vector::R`](R) reader structure"]
impl crate::Readable for EccVectorSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_vector::W`](W) writer structure"]
impl crate::Writable for EccVectorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_VECTOR to value 0"]
impl crate::Resettable for EccVectorSpec {
    const RESET_VALUE: u32 = 0;
}
