#[doc = "Register `HW_SPARE_RW0` reader"]
pub type R = crate::R<HwSpareRw0Spec>;
#[doc = "Register `HW_SPARE_RW0` writer"]
pub type W = crate::W<HwSpareRw0Spec>;
#[doc = "Field `hw_spare_rw0` reader - 31:0\\]
Bit 0: Writing 1'b1 will mask the hwa local ram agg serr propagation to ESM Writing 1'b0 will unmask the hwa local ram agg serr propagation to ESM Bit 1 : Writing 1'b1 will mask the hwa local ram agg uerr propagation to ESM Writing 1'b0 will unmask the hwa local ram agg uerr propagation to ESM Bit 2 to 31 Reserved"]
pub type HwSpareRw0R = crate::FieldReader<u32>;
#[doc = "Field `hw_spare_rw0` writer - 31:0\\]
Bit 0: Writing 1'b1 will mask the hwa local ram agg serr propagation to ESM Writing 1'b0 will unmask the hwa local ram agg serr propagation to ESM Bit 1 : Writing 1'b1 will mask the hwa local ram agg uerr propagation to ESM Writing 1'b0 will unmask the hwa local ram agg uerr propagation to ESM Bit 2 to 31 Reserved"]
pub type HwSpareRw0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bit 0: Writing 1'b1 will mask the hwa local ram agg serr propagation to ESM Writing 1'b0 will unmask the hwa local ram agg serr propagation to ESM Bit 1 : Writing 1'b1 will mask the hwa local ram agg uerr propagation to ESM Writing 1'b0 will unmask the hwa local ram agg uerr propagation to ESM Bit 2 to 31 Reserved"]
    #[inline(always)]
    pub fn hw_spare_rw0(&self) -> HwSpareRw0R {
        HwSpareRw0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bit 0: Writing 1'b1 will mask the hwa local ram agg serr propagation to ESM Writing 1'b0 will unmask the hwa local ram agg serr propagation to ESM Bit 1 : Writing 1'b1 will mask the hwa local ram agg uerr propagation to ESM Writing 1'b0 will unmask the hwa local ram agg uerr propagation to ESM Bit 2 to 31 Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn hw_spare_rw0(&mut self) -> HwSpareRw0W<HwSpareRw0Spec> {
        HwSpareRw0W::new(self, 0)
    }
}
#[doc = "HW_SPARE_RW0\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_spare_rw0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_spare_rw0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwSpareRw0Spec;
impl crate::RegisterSpec for HwSpareRw0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_spare_rw0::R`](R) reader structure"]
impl crate::Readable for HwSpareRw0Spec {}
#[doc = "`write(|w| ..)` method takes [`hw_spare_rw0::W`](W) writer structure"]
impl crate::Writable for HwSpareRw0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_SPARE_RW0 to value 0"]
impl crate::Resettable for HwSpareRw0Spec {
    const RESET_VALUE: u32 = 0;
}
