#[doc = "Register `VECTOR` reader"]
pub type R = crate::R<VectorSpec>;
#[doc = "Register `VECTOR` writer"]
pub type W = crate::W<VectorSpec>;
#[doc = "Field `ECC_VEC` reader - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
pub type EccVecR = crate::FieldReader<u16>;
#[doc = "Field `ECC_VEC` writer - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
pub type EccVecW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `NU0` reader - 14:11\\]
Reserved"]
pub type Nu0R = crate::FieldReader;
#[doc = "Field `NU0` writer - 14:11\\]
Reserved"]
pub type Nu0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RD_SVBUS` reader - 15:15\\]
Write 1 to trigger a read on the serial VBUS. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type RdSvbusR = crate::BitReader;
#[doc = "Field `RD_SVBUS` writer - 15:15\\]
Write 1 to trigger a read on the serial VBUS. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type RdSvbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_SVBUS_ADDR` reader - 23:16\\]
Read address"]
pub type RdSvbusAddrR = crate::FieldReader;
#[doc = "Field `RD_SVBUS_ADDR` writer - 23:16\\]
Read address"]
pub type RdSvbusAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD_SVBUS_DONE` reader - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field."]
pub type RdSvbusDoneR = crate::BitReader;
#[doc = "Field `RD_SVBUS_DONE` writer - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field."]
pub type RdSvbusDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - 31:25\\]
Reserved"]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - 31:25\\]
Reserved"]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
    #[inline(always)]
    pub fn ecc_vec(&self) -> EccVecR {
        EccVecR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - 14:11\\]
Reserved"]
    #[inline(always)]
    pub fn nu0(&self) -> Nu0R {
        Nu0R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to trigger a read on the serial VBUS. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    pub fn rd_svbus(&self) -> RdSvbusR {
        RdSvbusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Read address"]
    #[inline(always)]
    pub fn rd_svbus_addr(&self) -> RdSvbusAddrR {
        RdSvbusAddrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field."]
    #[inline(always)]
    pub fn rd_svbus_done(&self) -> RdSvbusDoneR {
        RdSvbusDoneR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Value written to select the corresponding ECC RAM for control or status"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_vec(&mut self) -> EccVecW<VectorSpec> {
        EccVecW::new(self, 0)
    }
    #[doc = "Bits 11:14 - 14:11\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu0(&mut self) -> Nu0W<VectorSpec> {
        Nu0W::new(self, 11)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to trigger a read on the serial VBUS. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    #[must_use]
    pub fn rd_svbus(&mut self) -> RdSvbusW<VectorSpec> {
        RdSvbusW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Read address"]
    #[inline(always)]
    #[must_use]
    pub fn rd_svbus_addr(&mut self) -> RdSvbusAddrW<VectorSpec> {
        RdSvbusAddrW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Status to indicate if read on serial VBUS is complete, write of any value will clear this bit. Writing 1 to any bit will clear the corresponding bits. Reads do not alter the value of the field."]
    #[inline(always)]
    #[must_use]
    pub fn rd_svbus_done(&mut self) -> RdSvbusDoneW<VectorSpec> {
        RdSvbusDoneW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<VectorSpec> {
        Nu1W::new(self, 25)
    }
}
#[doc = "ECC Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vector::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vector::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VectorSpec;
impl crate::RegisterSpec for VectorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vector::R`](R) reader structure"]
impl crate::Readable for VectorSpec {}
#[doc = "`write(|w| ..)` method takes [`vector::W`](W) writer structure"]
impl crate::Writable for VectorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VECTOR to value 0"]
impl crate::Resettable for VectorSpec {
    const RESET_VALUE: u32 = 0;
}
